#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, dispatch::DispatchResult};
use frame_system::{self as system, ensure_signed,
                   offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer}, };
use sp_std::prelude::*;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    offchain,
    offchain::{
        storage::StorageValueRef,
    },
    transaction_validity::{TransactionPriority}, SaturatedConversion};
use core::convert::TryInto;
use sp_runtime::traits::Saturating;

use sp_std::prelude::*;
use sp_std::str;

// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};
use sp_runtime::offchain::http::PendingRequest;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//This is the application key to be used as the prefix for this pallet in underlying storage.
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub const HTTP_REMOTE_COIN_CAP_URL_BYTES: &[u8] = b"https://api.coincap.io/v2/assets/ethereum";
pub const HTTP_REMOTE_COIN_GECKO_URL_BYTES: &[u8] = b"https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
pub const HTTP_REMOTE_CRYPT_COIN_URL_BYTES: &[u8] = b"";

pub const FETCH_TIMEOUT_PERIOD: u64 = 6000; // in milli-seconds
pub const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 1000; // in milli-seconds
pub const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number

pub mod crypto {
    use crate::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;

    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };

    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

    // implemented for ocw-runtime
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
        type RuntimeAppPublic = Public;
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }

    // implemented for mock runtime in test
    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
    for TestAuthId
    {
        type RuntimeAppPublic = Public;
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }
}

// Specifying serde path as `alt_serde`
// ref: https://serde.rs/container-attrs.html#crate
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Default)]
struct CoinCapInfo {
    data: DataInfo,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Default)]
struct DataInfo {
    #[serde(deserialize_with = "de_string_to_f64")]
    priceUsd: f64,
}


#[serde(crate = "alt_serde")]
#[derive(Deserialize, Default)]
struct CoinGeckoInfo {
    ethereum: GeckPriceInfo
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Default)]
struct GeckPriceInfo {
    usd: f64,
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}

pub fn de_string_to_f64<'de, D>(de: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.parse::<f64>().unwrap())
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
    /// The type to sign and send transactions.
    type UnsignedPriority: Get<TransactionPriority>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(fn something): Option<u32>;

		Number get(fn number):  Option<u64>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),

		NumberStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
		SubmitNumberSignedError,
		// Error returned when making remote http fetching
		HttpFetchingError,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");
			let price_result = Self::fetch_coin_price();
			match price_result {
			    Ok(price) => {
			        debug::info!("offchain_worker begin to invoke save price: {}", price);
                    Self::save_price(price);
			    }
			    Err(err) => {
			        debug::error!("offchain_worker get error: {:?}", err);
			    }
			}
		}

	}
}


impl<T: Trait> Module<T> {
    fn save_price(price: u64) {
        debug::info!("save price to storage: {}", price);
        let price_info = StorageValueRef::persistent(b"offchain-demo::price-info");
        let mut price_vec = Vec::<u64>::new();

        if let Some(Some(original_price_vec)) = price_info.get::<Vec<u64>>() {
            debug::info!("get original price vec>>>>");
            price_vec = original_price_vec;
        }
        debug::info!("push new price to vec and set to storage>>>>");
        price_vec.push(price);
        debug::info!("now the price vec values: {:?}", price_vec);
        price_info.set(&price_vec);
    }

    //构造请求组
    fn build_coin_request(url_bytes: &[u8]) -> Result<PendingRequest, Error<T>> {
        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp().add(offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

        let remote_url_bytes = url_bytes.to_vec();
        let remote_url = str::from_utf8(&remote_url_bytes)
            .map_err(|_| <Error<T>>::HttpFetchingError)?;
        debug::info!("sending request to: {}", remote_url);

        // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
        let request = offchain::http::Request::get(remote_url);

        let pending_request = request
            .deadline(timeout) // Setting the timeout time
            .send() // Sending the request out by the host
            .map_err(|_| <Error<T>>::HttpFetchingError)?;

        Ok(pending_request)

    }

    fn fetch_coin_price() -> Result<u64, Error<T>> {

        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp().add(offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

        let coin_cap_pending_request = Self::build_coin_request(HTTP_REMOTE_COIN_CAP_URL_BYTES).map_err(|_| <Error<T>>::HttpFetchingError)?;
        let coin_gecko_request = Self::build_coin_request(HTTP_REMOTE_COIN_GECKO_URL_BYTES).map_err(|_| <Error<T>>::HttpFetchingError)?;

        let mut request_vec = Vec::<PendingRequest>::new();
        request_vec.push(coin_cap_pending_request);
        request_vec.push(coin_gecko_request);

        let mut result_vec = PendingRequest::try_wait_all(request_vec, timeout);

        debug::info!("get all response>>>>");

        //检查返回结果
        if result_vec.len() == 0 {
            //返回错误
            debug::error!("result vec is 0>>>>>");
            return Err(<Error<T>>::HttpFetchingError);
        } else {
            debug::info!("begin to parse response>>>>");
            //取得第一个结果
            let coin_cap_response = result_vec.remove(0)
                .map_err(|_| <Error<T>>::HttpFetchingError)?
                .map_err(|_| <Error<T>>::HttpFetchingError)?;
            //取得第二个结果
            let coin_gecko_response = result_vec.remove(0)
                .map_err(|_| <Error<T>>::HttpFetchingError)?
                .map_err(|_| <Error<T>>::HttpFetchingError)?;

            if coin_cap_response.code == 200 && coin_gecko_response.code == 200 {

                debug::info!("all code is 200>>>>");

                //必须两个都为200 OK才行
                let coin_cap_result_bytes = coin_cap_response.body().collect::<Vec<u8>>();
                let coin_gecko_result_bytes = coin_gecko_response.body().collect::<Vec<u8>>();

                debug::info!("begin to convert to string>>>>");

                let coin_cap_resp_str = str::from_utf8(&coin_cap_result_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
                let coin_gecko_resp_str = str::from_utf8(&coin_gecko_result_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;

                // Print out our fetched JSON string
                debug::info!("coin cap resp: {}", coin_cap_resp_str);
                debug::info!("coin gecko resp: {}", coin_gecko_resp_str);

                let coin_cap_info: CoinCapInfo = serde_json::from_str(&coin_cap_resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
                let eth_price_1 = coin_cap_info.data.priceUsd as u64;

                let coin_gecko_info: CoinGeckoInfo = serde_json::from_str(&coin_gecko_resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
                let eth_price_2 = coin_gecko_info.ethereum.usd as u64;

                debug::info!("coin cap price1: {}, price2: {}", eth_price_1, eth_price_2);

                let final_price = (eth_price_1 + eth_price_2) / 2;

                return Ok(final_price);
            } else {
                debug::error!("Coin cap or coin gecko unexpected http request status code: coin_cap_code: {}, coin_gecko_code: {}", coin_cap_response.code, coin_gecko_response.code);
                return Err(<Error<T>>::HttpFetchingError);
            }
        }
    }
}
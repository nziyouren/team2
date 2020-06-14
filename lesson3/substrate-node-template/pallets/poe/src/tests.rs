// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};


#[test]
fn create_claim_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2,3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    })
}

//test case for claim already existed
#[test]
fn create_claim_alreay_existed() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2,3];
        //insert first time
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        //insert same claim again
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
    })
}




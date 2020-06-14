// Tests to be written here

use crate::{Error, mock::*};
use super::*;
use frame_support::{assert_ok, assert_noop};


#[test]
fn create_claim_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    })
}

//test case for claim already existed
#[test]
fn create_claim_alreay_existed() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        //insert first time
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        //insert same claim again
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
    })
}

//test case for claim too long
#[test]
fn create_claim_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3, 4, 5, 6];
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofTooLong);
    })
}


//test case for revoke claim
#[test]
fn revoke_claim_ok() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        //insert first time
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}


//test case for revoke claim not exsited
#[test]
fn revoke_claim_not_existed() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        //insert first time
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let another_claim = vec![4, 4, 4, 4];
        assert_noop!(PoeModule::revoke_claim(Origin::signed(1), another_claim.clone()), Error::<Test>::ClaimNotExist);
    })
}

//test case for revoke claim not owner
#[test]
fn revoke_claim_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2, 3];
        //insert first time
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotClaimOwner);
    })
}




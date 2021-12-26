use frame_support::{assert_noop, assert_ok};

use crate::{Error, mock::*, Proofs};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1];
		assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(1, frame_system::Pallet::<Test>::block_number())
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1];
		let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			TemplateModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	})
}

#[test]
fn create_claim_failed_when_input_is_too_short() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_noop!(TemplateModule::create_claim(Origin::signed(1), claim.clone()),
		Error::<Test>::TooShort);
	})
}

#[test]
fn create_claim_failed_when_input_is_too_long() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
		assert_noop!(TemplateModule::create_claim(Origin::signed(1), claim.clone()),
		Error::<Test>::TooLong);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1];
		let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(TemplateModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), (0, 0))
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1];
		assert_noop!(TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
		Error::<Test>::NoSuchProof);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1];
		let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(TemplateModule::transfer_clain(Origin::signed(1), claim.clone(), 12 as u64));
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_owned() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 2];
		assert_noop!(TemplateModule::transfer_clain(Origin::signed(1), claim.clone(), 21 as u64),
			Error::<Test>::NotProofOwner);
	})
}

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }
//
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }

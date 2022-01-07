use frame_support::{assert_noop, assert_ok};

use crate::{Error, mock::*};

#[test]
fn create_kitty_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 0, 1, 0, 1, 0, 1];
		assert_ok!(TemplateModule::create(Origin::signed(11)));
	})
}

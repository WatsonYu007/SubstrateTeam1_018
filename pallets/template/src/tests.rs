use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::{
	traits::{ Header as _ },
};

use frame_support::traits::{ OnInitialize};
fn setup_blocks(blocks: u64) {
	let mut parent_hash = System::parent_hash();
	for i in 1..(blocks + 1) {
		System::initialize(&i, &parent_hash, &Default::default(), frame_system::InitKind::Full);
		RandomnessCollectiveFlip::on_initialize(i);

		let header = System::finalize();
		parent_hash = header.hash();
		System::set_block_number(*header.number());
	}
}

#[test]
fn create_kitty_should_work() {
	new_test_ext().execute_with(|| {

		setup_blocks(80);
		assert_ok!(Balances::set_balance(Origin::root(), 1, 10000000, 0));
		assert_eq!(Balances::free_balance(&1), 10000000);
		// create a kitty with account #10.
		let kitty00 = SubstrateKitties::create(Origin::signed(10));
		assert_ok!(kitty00);
		assert_ok!(SubstrateKitties::create(Origin::signed(10)));
		assert_ok!(SubstrateKitties::create(Origin::signed(10)));
		// check that there is now 3 kitty in storage
		assert_eq!(SubstrateKitties::kitties_count(), Some(3));
	});
}

#[test]
fn transfer_kitty_should_work() {
	new_test_ext().execute_with(|| {

		setup_blocks(80);
		assert_ok!(Balances::set_balance(Origin::root(), 1, 10000000, 0));
		assert_eq!(Balances::free_balance(&1), 10000000);
		// create a kitty with account #10.
		let from = Origin::signed(10);
		let to = Origin::signed(11);
		let kitty00 = SubstrateKitties::create(from.clone());
		assert_ok!(kitty00);
		let kitty_id = SubstrateKitties::kitties_count().unwrap();
		assert_ok!(SubstrateKitties::transfer(from, 3, kitty_id));

		assert_eq!(SubstrateKitties::owner_of(kitty_id), Some(3));
		// check that there is now 3 kitty in storage
	});
}

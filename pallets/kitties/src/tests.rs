#![cfg(test)]

use crate::{
	mock::*, pallet::{Error}
};
use frame_support::{assert_ok, assert_noop};

#[test]
fn should_build_genesis_kitties() {
	new_test_ext().execute_with(|| {
		// Check we have 2 kitties, as specified
		assert_eq!(SubstrateKitties::kitty_cnt(), 2);

		// Check owners own the correct amount of kitties
		let kitties_owned_by_1 = SubstrateKitties::kitties_owned(1);
		assert_eq!(kitties_owned_by_1.len(), 1);

		let kitties_owned_by_2 = SubstrateKitties::kitties_owned(2);
		assert_eq!(kitties_owned_by_2.len(), 1);

		// Check that kitties are owned correctly
		let kid1 = kitties_owned_by_1[0];
		let kitty1 = SubstrateKitties::kitties(kid1)
			.expect("Could have this kitty ID owned by acct 1");
		assert_eq!(kitty1.owner, 1);

		let kid2 = kitties_owned_by_2[0];
		let kitty2 = SubstrateKitties::kitties(kid2)
			.expect("Could have this kitty ID owned by acct 2");
		assert_eq!(kitty2.owner, 2);
	});
}

#[test]
fn create_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		// check that 3 kitties exists (together with the two from genesis)
		assert_eq!(SubstrateKitties::kitty_cnt(), 3);

		// check that account #10 owns 1 kitty
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 1);

		// check that some random account #5 does not own a kitty
		assert_eq!(SubstrateKitties::kitties_owned(5).len(), 0);

		// check that this kitty is specifically owned by account #10
		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);
	});
}

#[test]
fn create_kitty_failed_when_not_have_enough_balance() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		// account #0 cannot create a kitty with not have enough balance to reserve.
		assert_noop!(
			SubstrateKitties::create_kitty(Origin::signed(0)),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn set_price_should_work() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(10), hash, Some(5)));

		let new_kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(new_kitty.owner, 10);
		assert_eq!(new_kitty.price, Some(5));
	});
}

#[test]
fn set_price_failed_with_not_kitty_owner() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(10), hash, Some(5)));

		assert_noop!(
			SubstrateKitties::set_price(Origin::signed(1), hash, Some(10)),
			Error::<Test>::NotKittyOwner
		);
	});
}

#[test]
fn transfer_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// check that acct 10 own a kitty
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 1);
		let hash = SubstrateKitties::kitties_owned(10)[0];

		// acct 10 send kitty to acct 3
		assert_ok!(SubstrateKitties::transfer(Origin::signed(10), 3, hash));

		// acct 10 now has nothing
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 0);
		// but acct 3 does
		assert_eq!(SubstrateKitties::kitties_owned(3).len(), 1);
		let new_hash = SubstrateKitties::kitties_owned(3)[0];
		// and it has the same hash
		assert_eq!(hash, new_hash);
	});
}

#[test]
fn transfer_non_owned_kitty_should_fail() {
	new_test_ext().execute_with(|| {
		let hash = SubstrateKitties::kitties_owned(1)[0];

		// account 0 cannot transfer a kitty with this hash.
		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(9), 2, hash),
			Error::<Test>::NotKittyOwner
		);
	});
}

#[test]
fn transfer_failed_with_capacity_exceeded() {
	new_test_ext().execute_with(|| {
		// check that acct 10 own a kitty
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 1);
		let hash_1 = SubstrateKitties::kitties_owned(10)[0];

		// acct 10 send kitty to acct 4
		assert_ok!(SubstrateKitties::transfer(Origin::signed(10), 4, hash_1));

		let mut index = 0;
		while index < 19 {
			assert_ok!(SubstrateKitties::create_kitty(Origin::signed(4)));
			index = index + 1;
		}

		assert_eq!(SubstrateKitties::kitties_owned(4).len(), 20);

		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 1);
		let hash_2 = SubstrateKitties::kitties_owned(10)[0];

		// account 0 cannot transfer a kitty with this hash.
		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(10), 4, hash_2),
			Error::<Test>::ExceedMaxKittyOwned
		);
	});
}

#[test]
fn buy_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(10), hash, Some(5)));

		let new_kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(new_kitty.owner, 10);
		assert_eq!(new_kitty.price, Some(5));

		assert_eq!(SubstrateKitties::kitties_owned(1).len(), 1);
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(1), hash, 5));
		assert_eq!(SubstrateKitties::kitties_owned(1).len(), 2);
	});
}

#[test]
fn buy_kitty_failed_when_buyer_is_owner() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(10), hash, Some(5)));

		let new_kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(new_kitty.owner, 10);
		assert_eq!(new_kitty.price, Some(5));

		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(10), hash, 5),
			Error::<Test>::BuyerIsKittyOwner
		);
	});
}

#[test]
fn buy_kitty_failed_with_low_bid() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(10), hash, Some(5)));

		let new_kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(new_kitty.owner, 10);
		assert_eq!(new_kitty.price, Some(5));

		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(1), hash, 3),
			Error::<Test>::KittyBidPriceTooLow
		);
	});
}

#[test]
fn buy_kitty_failed_with_low_balance() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(10), hash, Some(20)));

		let new_kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(new_kitty.owner, 10);
		assert_eq!(new_kitty.price, Some(20));

		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(1), hash, 20),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn breed_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash_1 = SubstrateKitties::kitties_owned(10)[0];
		let _kitty_1 = SubstrateKitties::kitties(hash_1).expect("should found the kitty");

		// create another kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash_2 = SubstrateKitties::kitties_owned(10)[1];
		let _kitty_2 = SubstrateKitties::kitties(hash_2).expect("should found the kitty");

		assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(10), hash_1, hash_2));
	});
}

#[test]
fn breed_kitty_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash_1 = SubstrateKitties::kitties_owned(10)[0];
		let _kitty_1 = SubstrateKitties::kitties(hash_1).expect("should found the kitty");

		// create another kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		let hash_2 = SubstrateKitties::kitties_owned(10)[1];
		let _kitty_2 = SubstrateKitties::kitties(hash_2).expect("should found the kitty");

		assert_noop!(
			SubstrateKitties::breed_kitty(Origin::signed(1), hash_1, hash_2),
			Error::<Test>::NotKittyOwner
		);
	});
}
#[allow(unused)]
use frame_support::{assert_err, assert_noop, assert_ok};
#[allow(unused)]
use frame_system::Account;
#[allow(unused)]
use primitives::*;
#[allow(unused)]
use crate::{mock::*, BalanceOf, Config, Error, Event, Workers};

#[allow(unused)]
const ALICE: AccountId = 1;
#[allow(unused)]
const ALICE_WORKER: AccountId = 2;
#[allow(unused)]
const BOB: AccountId = 3;
#[allow(unused)]
const BOB_WORKER: AccountId = 4;

type WorkerInfo = primitives::WorkerInfo<<Test as frame_system::Config>::AccountId, BalanceOf<Test>, u32>;

fn register_worker_for(owner: AccountId, worker: AccountId, initial_deposit: Balance) -> WorkerInfo {
	let owner_balance = Balances::free_balance(owner);

	assert_ok!(
		OffchainComputingWorkers::register_worker(
			RuntimeOrigin::signed(owner),
			worker.into(),
			initial_deposit
		)
	);

	let worker_info = Workers::<Test>::get(worker).unwrap();

	assert_eq!(worker_info.status, WorkerStatus::Registered);
	assert_eq!(Balances::free_balance(owner), owner_balance.saturating_sub(initial_deposit));
	assert_eq!(Balances::reserved_balance(worker), worker_info.deposit);
	assert_eq!(Balances::free_balance(worker), initial_deposit.saturating_sub(worker_info.deposit));

	worker_info
}

#[test]
fn register_worker_works() {
	new_test_ext().execute_with(|| {
		set_balance(ALICE, 201 * DOLLARS);

		register_worker_for(ALICE, ALICE_WORKER, 101 * DOLLARS);

		run_to_block(1);
		set_balance(ALICE, 201 * DOLLARS);

		assert_noop!(
			OffchainComputingWorkers::register_worker(RuntimeOrigin::signed(ALICE), ALICE_WORKER, 11 * DOLLARS),
			Error::<Test>::InitialBalanceTooLow
		);

		assert_noop!(
			OffchainComputingWorkers::register_worker(RuntimeOrigin::signed(ALICE), ALICE_WORKER, 101 * DOLLARS),
			Error::<Test>::AlreadyRegistered
		);
	});
}

#[test]
fn deregister_worker_works() {
	new_test_ext().execute_with(|| {
		set_balance(ALICE, 201 * DOLLARS);

		register_worker_for(ALICE, ALICE_WORKER, 101 * DOLLARS);

		run_to_block(1);

		assert_ok!(OffchainComputingWorkers::deregister_worker(RuntimeOrigin::signed(ALICE), ALICE_WORKER));

		assert_eq!(Balances::free_balance(ALICE), 201 * DOLLARS);
		assert_eq!(Account::<Test>::contains_key(ALICE_WORKER), false);
	});
}

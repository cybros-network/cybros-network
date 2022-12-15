#[allow(unused)]
use crate::{mock::*, types::*, Config, Error, Event};
#[allow(unused)]
use frame_support::{assert_err, assert_noop, assert_ok};
#[allow(unused)]
use frame_system::Account;

#[allow(unused)]
const ALICE: AccountId = 1;
#[allow(unused)]
const ALICE_WORKER: AccountId = 2;
#[allow(unused)]
const BOB: AccountId = 3;
#[allow(unused)]
const BOB_WORKER: AccountId = 4;

fn register_worker_for(owner: AccountId, worker: AccountId, initial_deposit: Balance) -> WorkerInfo<Test> {
	let owner_balance = Balances::free_balance(owner);

	assert_ok!(ComputingWorkers::register(RuntimeOrigin::signed(owner), worker, initial_deposit));

	let worker_info = ComputingWorkers::workers(worker).unwrap();

	assert_eq!(worker_info.status, WorkerStatus::Registered);
	assert_eq!(Balances::free_balance(owner), owner_balance - initial_deposit);
	assert_eq!(Balances::reserved_balance(worker), worker_info.reserved);
	assert_eq!(Balances::free_balance(worker), initial_deposit - worker_info.reserved);

	worker_info
}

#[test]
fn register_works() {
	new_test_ext().execute_with(|| {
		set_balance(ALICE, 101 * DOLLARS, 0);

		register_worker_for(ALICE, ALICE_WORKER, 100 * DOLLARS);

		run_to_block(1);
		set_balance(ALICE, 101 * DOLLARS, 0);

		assert_noop!(
			ComputingWorkers::register(RuntimeOrigin::signed(ALICE), ALICE_WORKER, 10 * DOLLARS),
			Error::<Test>::InitialDepositTooLow
		);

		assert_noop!(
			ComputingWorkers::register(RuntimeOrigin::signed(ALICE), ALICE_WORKER, 100 * DOLLARS),
			Error::<Test>::AlreadyRegistered
		);
	});
}

#[test]
fn deregister_works() {
	new_test_ext().execute_with(|| {
		set_balance(ALICE, 101 * DOLLARS, 0);

		register_worker_for(ALICE, ALICE_WORKER, 100 * DOLLARS);

		run_to_block(1);

		assert_ok!(ComputingWorkers::deregister(RuntimeOrigin::signed(ALICE), ALICE_WORKER));

		assert_eq!(Balances::free_balance(ALICE), 101 * DOLLARS);
		assert_eq!(Account::<Test>::contains_key(ALICE_WORKER), false);
	});
}

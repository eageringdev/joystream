// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for working_group
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-13, STEPS: `10`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=working_group
// --extrinsic=*
// --chain=dev
// --steps=10
// --repeat=5
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for working_group.
pub trait WeightInfo {
	fn on_initialize_leaving(_i: u32, ) -> Weight;
	fn on_initialize_rewarding_with_missing_reward(_i: u32, ) -> Weight;
	fn on_initialize_rewarding_with_missing_reward_cant_pay(_i: u32, ) -> Weight;
	fn on_initialize_rewarding_without_missing_reward(_i: u32, ) -> Weight;
	fn apply_on_opening(_i: u32, ) -> Weight;
	fn fill_opening_lead() -> Weight;
	fn fill_opening_worker(_i: u32, ) -> Weight;
	fn update_role_account() -> Weight;
	fn cancel_opening() -> Weight;
	fn withdraw_application() -> Weight;
	fn slash_stake(_i: u32, ) -> Weight;
	fn terminate_role_worker(_i: u32, ) -> Weight;
	fn terminate_role_lead(_i: u32, ) -> Weight;
	fn increase_stake() -> Weight;
	fn decrease_stake() -> Weight;
	fn spend_from_budget() -> Weight;
	fn fund_working_group_budget() -> Weight;
	fn update_reward_amount() -> Weight;
	fn set_status_text(_i: u32, ) -> Weight;
	fn update_reward_account() -> Weight;
	fn set_budget() -> Weight;
	fn add_opening(_i: u32, ) -> Weight;
	fn leave_role(_i: u32, ) -> Weight;
	fn lead_remark() -> Weight;
	fn worker_remark() -> Weight;
}

/// Weights for working_group using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:2)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	// Storage: Balances Locks (r:2 w:2)
	fn on_initialize_leaving(i: u32, ) -> Weight {
		(28_376_000 as Weight)
			// Standard Error: 187_000
			.saturating_add((55_473_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:2)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn on_initialize_rewarding_with_missing_reward(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 279_000
			.saturating_add((53_720_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:2)
	// Storage: Instance3WorkingGroup Budget (r:1 w:0)
	fn on_initialize_rewarding_with_missing_reward_cant_pay(i: u32, ) -> Weight {
		(35_545_000 as Weight)
			// Standard Error: 101_000
			.saturating_add((22_665_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:3 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn on_initialize_rewarding_without_missing_reward(i: u32, ) -> Weight {
		(40_312_000 as Weight)
			// Standard Error: 129_000
			.saturating_add((32_326_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:0)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup NextApplicationId (r:1 w:1)
	// Storage: Instance3WorkingGroup ApplicationById (r:0 w:1)
	fn apply_on_opening(i: u32, ) -> Weight {
		(65_518_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:1)
	// Storage: Instance3WorkingGroup ApplicationById (r:1 w:1)
	// Storage: Instance3WorkingGroup NextWorkerId (r:1 w:1)
	// Storage: Instance3WorkingGroup WorkerById (r:0 w:1)
	fn fill_opening_lead() -> Weight {
		(54_493_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:1)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	// Storage: Instance3WorkingGroup ApplicationById (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup NextWorkerId (r:1 w:1)
	fn fill_opening_worker(i: u32, ) -> Weight {
		(65_703_000 as Weight)
			// Standard Error: 91_000
			.saturating_add((14_506_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	fn update_role_account() -> Weight {
		(32_021_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance3WorkingGroup OpeningById (r:1 w:1)
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_opening() -> Weight {
		(61_685_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance3WorkingGroup ApplicationById (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn withdraw_application() -> Weight {
		(41_304_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn slash_stake(i: u32, ) -> Weight {
		(80_899_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	fn terminate_role_worker(i: u32, ) -> Weight {
		(129_349_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:1)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: Instance3WorkingGroup ActiveWorkerCount (r:1 w:1)
	fn terminate_role_lead(i: u32, ) -> Weight {
		(127_720_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn increase_stake() -> Weight {
		(53_179_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn decrease_stake() -> Weight {
		(61_487_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn spend_from_budget() -> Weight {
		(46_705_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup Budget (r:1 w:1)
	fn fund_working_group_budget() -> Weight {
		(45_673_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:2 w:1)
	fn update_reward_amount() -> Weight {
		(39_044_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Instance3WorkingGroup StatusTextHash (r:0 w:1)
	fn set_status_text(i: u32, ) -> Weight {
		(32_202_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	fn update_reward_account() -> Weight {
		(28_906_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance3WorkingGroup Budget (r:0 w:1)
	fn set_budget() -> Weight {
		(15_534_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Instance3WorkingGroup NextOpeningId (r:1 w:1)
	// Storage: Instance3WorkingGroup OpeningById (r:0 w:1)
	fn add_opening(i: u32, ) -> Weight {
		(78_953_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:1)
	fn leave_role(i: u32, ) -> Weight {
		(33_087_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance3WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	fn lead_remark() -> Weight {
		(28_915_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance3WorkingGroup WorkerById (r:1 w:0)
	fn worker_remark() -> Weight {
		(27_169_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn on_initialize_leaving(i: u32, ) -> Weight {
		0
	}
	fn on_initialize_rewarding_with_missing_reward(i: u32, ) -> Weight {
		0
	}
	fn on_initialize_rewarding_with_missing_reward_cant_pay(i: u32, ) -> Weight {
		0
	}
	fn on_initialize_rewarding_without_missing_reward(i: u32, ) -> Weight {
		0
	}
	fn apply_on_opening(i: u32, ) -> Weight {
		0
	}
	fn fill_opening_lead() -> Weight {
		0
	}
	fn fill_opening_worker(i: u32, ) -> Weight {
		0
	}
	fn update_role_account() -> Weight {
		0
	}
	fn cancel_opening() -> Weight {
		0
	}
	fn withdraw_application() -> Weight {
		0
	}
	fn slash_stake(i: u32, ) -> Weight {
		0
	}
	fn terminate_role_worker(i: u32, ) -> Weight {
		0
	}
	fn terminate_role_lead(i: u32, ) -> Weight {
		0
	}
	fn increase_stake() -> Weight {
		0
	}
	fn decrease_stake() -> Weight {
		0
	}
	fn spend_from_budget() -> Weight {
		0
	}
	fn fund_working_group_budget() -> Weight {
		0
	}
	fn update_reward_amount() -> Weight {
		0
	}
	fn set_status_text(i: u32, ) -> Weight {
		0
	}
	fn update_reward_account() -> Weight {
		0
	}
	fn set_budget() -> Weight {
		0
	}
	fn add_opening(i: u32, ) -> Weight {
		0
	}
	fn leave_role(i: u32, ) -> Weight {
		0
	}
	fn lead_remark() -> Weight {
		0
	}
	fn worker_remark() -> Weight {
		0
	}
}

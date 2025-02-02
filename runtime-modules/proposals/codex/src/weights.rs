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

//! Autogenerated weights for proposals_codex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --base-path=/mnt/disks/local-ssd/
// --pallet=proposals_codex
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/proposals/codex/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for proposals_codex.
pub trait WeightInfo {
	fn create_proposal_signal(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_runtime_upgrade(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_funding_request(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_max_validator_count(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_veto_proposal(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_create_working_group_lead_opening(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_fill_working_group_lead_opening(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_update_working_group_budget(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_decrease_working_group_lead_stake(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_slash_working_group_lead(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_working_group_lead_reward(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_terminate_working_group_lead(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_amend_constitution(_i: u32, _t: u32, _d: u32, ) -> Weight;
	fn create_proposal_cancel_working_group_lead_opening(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_membership_price(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_council_budget_increment(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_councilor_reward(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_initial_invitation_balance(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_initial_invitation_count(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_membership_lead_invitation_quota(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_set_referral_cut(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_update_global_nft_limit(_t: u32, _d: u32, ) -> Weight;
	fn create_proposal_update_channel_payouts(_t: u32, _d: u32, _i: u32, ) -> Weight;
}

/// Weights for proposals_codex using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_signal(i: u32, t: u32, d: u32, ) -> Weight {
		(98_033_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((1_283_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 39_000
			.saturating_add((468_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 39_000
			.saturating_add((570_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_runtime_upgrade(i: u32, t: u32, d: u32, ) -> Weight {
		(100_888_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((1_267_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 28_000
			.saturating_add((357_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 28_000
			.saturating_add((556_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_funding_request(i: u32, t: u32, d: u32, ) -> Weight {
		(81_491_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((109_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 23_000
			.saturating_add((936_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 23_000
			.saturating_add((1_048_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_max_validator_count(t: u32, d: u32, ) -> Weight {
		(84_690_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((803_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 17_000
			.saturating_add((983_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: ProposalEngine Proposals (r:1 w:1)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_veto_proposal(t: u32, d: u32, ) -> Weight {
		(89_632_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((803_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 15_000
			.saturating_add((980_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_create_working_group_lead_opening(i: u32, t: u32, d: u32, ) -> Weight {
		(112_039_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((1_387_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 44_000
			.saturating_add((1_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 44_000
			.saturating_add((520_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup OpeningById (r:1 w:0)
	// Storage: Instance1WorkingGroup ApplicationById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_fill_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		(95_002_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((839_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 18_000
			.saturating_add((998_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_update_working_group_budget(t: u32, d: u32, ) -> Weight {
		(81_310_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((801_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 17_000
			.saturating_add((973_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_decrease_working_group_lead_stake(t: u32, d: u32, ) -> Weight {
		(85_821_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((826_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 17_000
			.saturating_add((966_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_slash_working_group_lead(t: u32, d: u32, ) -> Weight {
		(86_524_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((794_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 18_000
			.saturating_add((952_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_working_group_lead_reward(t: u32, d: u32, ) -> Weight {
		(86_242_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((822_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 19_000
			.saturating_add((965_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_terminate_working_group_lead(t: u32, d: u32, ) -> Weight {
		(87_711_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((773_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 20_000
			.saturating_add((929_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_amend_constitution(i: u32, t: u32, _d: u32, ) -> Weight {
		(121_461_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((1_323_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 66_000
			.saturating_add((632_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Instance1WorkingGroup OpeningById (r:1 w:0)
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_cancel_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		(87_854_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((797_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 18_000
			.saturating_add((966_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_membership_price(t: u32, d: u32, ) -> Weight {
		(83_212_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((786_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 19_000
			.saturating_add((898_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_council_budget_increment(t: u32, d: u32, ) -> Weight {
		(82_873_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((768_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 18_000
			.saturating_add((917_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_councilor_reward(t: u32, d: u32, ) -> Weight {
		(81_134_000 as Weight)
			// Standard Error: 17_000
			.saturating_add((823_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 17_000
			.saturating_add((961_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_initial_invitation_balance(t: u32, d: u32, ) -> Weight {
		(81_787_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((786_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 16_000
			.saturating_add((962_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_initial_invitation_count(t: u32, d: u32, ) -> Weight {
		(81_029_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((839_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 19_000
			.saturating_add((970_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_membership_lead_invitation_quota(t: u32, d: u32, ) -> Weight {
		(82_509_000 as Weight)
			// Standard Error: 18_000
			.saturating_add((773_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 18_000
			.saturating_add((911_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_set_referral_cut(t: u32, d: u32, ) -> Weight {
		(78_373_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((932_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 14_000
			.saturating_add((1_049_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_update_global_nft_limit(t: u32, d: u32, ) -> Weight {
		(83_079_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((774_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 19_000
			.saturating_add((924_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Membership MembershipById (r:1 w:0)
	// Storage: ProposalEngine ActiveProposalCount (r:1 w:1)
	// Storage: Membership StakingAccountIdMemberStatus (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ProposalDiscussion ThreadCount (r:1 w:1)
	// Storage: ProposalEngine ProposalCount (r:1 w:1)
	// Storage: ProposalsCodex ThreadIdByProposalId (r:0 w:1)
	// Storage: ProposalDiscussion ThreadById (r:0 w:1)
	// Storage: ProposalEngine Proposals (r:0 w:1)
	// Storage: ProposalEngine DispatchableCallCode (r:0 w:1)
	fn create_proposal_update_channel_payouts(t: u32, d: u32, i: u32, ) -> Weight {
		(99_787_000 as Weight)
			// Standard Error: 38_000
			.saturating_add((440_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 38_000
			.saturating_add((601_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 5_000
			.saturating_add((1_358_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_proposal_signal(i: u32, t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_runtime_upgrade(i: u32, t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_funding_request(i: u32, t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_max_validator_count(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_veto_proposal(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_create_working_group_lead_opening(i: u32, t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_fill_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_update_working_group_budget(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_decrease_working_group_lead_stake(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_slash_working_group_lead(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_working_group_lead_reward(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_terminate_working_group_lead(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_amend_constitution(i: u32, t: u32, _d: u32, ) -> Weight {
		0
	}
	fn create_proposal_cancel_working_group_lead_opening(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_membership_price(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_council_budget_increment(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_councilor_reward(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_initial_invitation_balance(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_initial_invitation_count(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_membership_lead_invitation_quota(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_set_referral_cut(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_update_global_nft_limit(t: u32, d: u32, ) -> Weight {
		0
	}
	fn create_proposal_update_channel_payouts(t: u32, d: u32, i: u32, ) -> Weight {
		0
	}
}

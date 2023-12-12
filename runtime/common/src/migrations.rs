// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! # Migrations
//!
//! This module acts as a registry where each migration is defined. Each migration should implement
//! the "Migration" trait declared in the pallet-migrations crate.

#[cfg(feature = "try-runtime")]
use frame_support::ensure;
use frame_support::{
	pallet_prelude::GetStorageVersion,
	sp_runtime::traits::{Block, Header},
	traits::{Hash as PreimageHash, OnRuntimeUpgrade, PalletInfoAccess, StorageVersion},
	weights::Weight,
};
use pallet_author_slot_filter::Config as AuthorSlotFilterConfig;
use pallet_migrations::{GetMigrations, Migration};
use pallet_moonbeam_orbiters::CollatorsPool;
use pallet_parachain_staking::{Round, RoundInfo};
#[cfg(feature = "try-runtime")]
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "try-runtime")]
use sp_runtime::traits::Zero;
use sp_std::{marker::PhantomData, prelude::*};

pub struct PreimageMigrationHashToBoundedCall<T>(PhantomData<T>);
impl<T> Migration for PreimageMigrationHashToBoundedCall<T>
where
	T: pallet_preimage::Config<Hash = PreimageHash> + frame_system::Config,
{
	fn friendly_name(&self) -> &str {
		"MM_PreimageMigrationHashToBoundedCall"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		pallet_preimage::migration::v1::Migration::<T>::on_runtime_upgrade()
	}

	/// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
		pallet_preimage::migration::v1::Migration::<T>::pre_upgrade()
	}

	/// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		pallet_preimage::migration::v1::Migration::<T>::post_upgrade(state)
	}
}

pub struct PalletReferendaMigrateV0ToV1<T>(pub PhantomData<T>);
impl<T> Migration for PalletReferendaMigrateV0ToV1<T>
where
	T: pallet_referenda::Config<Hash = PreimageHash> + frame_system::Config,
{
	fn friendly_name(&self) -> &str {
		"MM_PalletReferendaMigrateV0ToV1"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		pallet_referenda::migration::v1::MigrateV0ToV1::<T>::on_runtime_upgrade()
	}

	/// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
		pallet_referenda::migration::v1::MigrateV0ToV1::<T>::pre_upgrade()
	}

	/// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		pallet_referenda::migration::v1::MigrateV0ToV1::<T>::post_upgrade(state)
	}
}

use pallet_xcm_transactor::{relay_indices::*, RelayIndices};
use sp_core::Get;
pub struct PopulateRelayIndices<T>(pub RelayChainIndices, pub PhantomData<T>);
impl<T: pallet_xcm_transactor::Config> Migration for PopulateRelayIndices<T> {
	fn friendly_name(&self) -> &str {
		"MM_PopulateRelayIndices"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		// insert input into storage
		RelayIndices::<T>::put(self.0);
		T::DbWeight::get().writes(1)
	}

	/// Run a standard pre-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
		// check storage is default pre migration
		assert_eq!(RelayIndices::<T>::get(), Default::default());
		Ok(Vec::new())
	}

	/// Run a standard post-runtime test. This works the same way as in a normal runtime upgrade.
	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		// check storage matches input post migration
		assert_eq!(RelayIndices::<T>::get(), self.0);
		Ok(())
	}
}

pub struct RemoveMinBondForOrbiterCollators<T>(pub PhantomData<T>);
impl<T> Migration for RemoveMinBondForOrbiterCollators<T>
where
	T: pallet_moonbeam_orbiters::Config,
	T: pallet_parachain_staking::Config,
	T: frame_system::Config,
{
	fn friendly_name(&self) -> &str {
		"MM_RemoveMinBondForOrbiterCollators"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		let mut weight = Weight::zero();
		CollatorsPool::<T>::iter_keys().for_each(|collator| {
			log::info!("Setting the bond for collator {:?} to zero", collator);
			weight += <pallet_parachain_staking::Pallet<T>>::set_candidate_bond_to_zero(&collator);
		});
		weight
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		CollatorsPool::<T>::iter_keys().for_each(|collator| {
			log::info!("Checking collator: {:?}", collator);
			let state = <pallet_parachain_staking::Pallet<T>>::candidate_info(&collator)
				.expect("collator should have candidate info");
			assert!(state.bond.is_zero(), "collator bond should be zero");
		});
		Ok(())
	}
}

pub struct UpdateFirstRoundRelayBlockNumber<T>(pub PhantomData<T>);
impl<T> Migration for UpdateFirstRoundRelayBlockNumber<T>
where
	T: pallet_parachain_staking::Config,
	T: cumulus_pallet_parachain_system::Config,
	T: frame_system::Config,
	u32: From<<<<T as frame_system::Config>::Block as Block>::Header as Header>::Number>,
{
	fn friendly_name(&self) -> &str {
		"MM_UpdateFirstRoundRelayBlockNumber"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		// First, fetch the last parachain block
		let para_block: u32 = frame_system::Pallet::<T>::block_number().into();

		// Fetch Round storage before the migration
		let round_info = pallet_parachain_staking::Pallet::<T>::round();

		// Calculate how many blocks have passed so far in the current round
		let para_block_diff = para_block.saturating_sub(round_info.first);

		// Calculate the percentage of the round so far (before the migration)
		let percentage = (para_block_diff)
			.saturating_mul(100)
			.saturating_div(round_info.length);

		// Calculate how many blocks should we substract from LastRelayChainBlockNumber
		// given the new duration (round_info.length * 2) to have a first relay block of the
		// round that corresponds with the percentage calculated in the step above.
		let new_block_diff = percentage
			.saturating_mul(round_info.length * 2)
			.saturating_div(100);

		// Perform substraction
		let new_first_block =
			cumulus_pallet_parachain_system::Pallet::<T>::last_relay_block_number()
				.saturating_sub(new_block_diff);

		// Update Round storage
		let new_round_info = RoundInfo {
			current: round_info.current,
			first: new_first_block,
			length: round_info.length * 2,
		};

		Round::<T>::put(new_round_info);

		T::DbWeight::get().reads_writes(2, 1)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
		let pre_round_info = pallet_parachain_staking::Pallet::<T>::round();
		Ok(pre_round_info.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self, state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		let pre_round_info = <RoundInfo<u32> as Decode>::decode(&mut &*state).unwrap_or_default();
		let post_round_info = pallet_parachain_staking::Pallet::<T>::round();
		assert_eq!(pre_round_info.length * 2, post_round_info.length);
		Ok(())
	}
}
pub struct MissingBalancesMigrations<T>(PhantomData<T>);
impl<T> Migration for MissingBalancesMigrations<T>
where
	T: pallet_balances::Config,
	<T as frame_system::Config>::AccountId: Default,
{
	fn friendly_name(&self) -> &str {
		"MM_MissingBalancesMigrations"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		pallet_balances::migration::MigrateToTrackInactive::<T, ()>::on_runtime_upgrade();
		pallet_balances::migration::ResetInactive::<T, ()>::on_runtime_upgrade();
		pallet_balances::migration::MigrateToTrackInactive::<T, ()>::on_runtime_upgrade()
	}
}

pub struct FixIncorrectPalletVersions<Runtime, Treasury, OpenTech>(
	pub PhantomData<(Runtime, Treasury, OpenTech)>,
);
impl<Runtime, Treasury, OpenTech> Migration
	for FixIncorrectPalletVersions<Runtime, Treasury, OpenTech>
where
	Treasury: GetStorageVersion + PalletInfoAccess,
	OpenTech: GetStorageVersion + PalletInfoAccess,
	Runtime: frame_system::Config,
	Runtime: pallet_referenda::Config,
{
	fn friendly_name(&self) -> &str {
		"MM_FixIncorrectPalletVersions"
	}

	fn migrate(&self, _available_weight: Weight) -> Weight {
		log::info!("Setting collective pallet versions to 4");
		StorageVersion::new(4).put::<Treasury>();
		StorageVersion::new(4).put::<OpenTech>();
		log::info!("Setting referenda pallet version to 1");
		StorageVersion::new(1).put::<pallet_referenda::Pallet<Runtime>>();
		Runtime::DbWeight::get().writes(2)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade(&self) -> Result<Vec<u8>, sp_runtime::DispatchError> {
		ensure!(
			<Treasury as GetStorageVersion>::on_chain_storage_version() == 0,
			"TreasuryCouncilCollective storage version should be 0"
		);
		ensure!(
			<OpenTech as GetStorageVersion>::on_chain_storage_version() == 0,
			"OpenTechCommitteeCollective storage version should be 0"
		);
		ensure!(
			<pallet_referenda::Pallet<Runtime> as GetStorageVersion>::on_chain_storage_version()
				== 0,
			"Referenda storage version should be 0"
		);
		Ok(vec![])
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(&self, _state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		ensure!(
			<Treasury as GetStorageVersion>::on_chain_storage_version() == 4,
			"Treasury storage version should be 4"
		);
		ensure!(
			<OpenTech as GetStorageVersion>::on_chain_storage_version() == 4,
			"OpenTech storage version should be 4"
		);
		ensure!(
			<pallet_referenda::Pallet<Runtime> as GetStorageVersion>::on_chain_storage_version()
				== 1,
			"Referenda storage version should be 1"
		);
		Ok(())
	}
}

pub struct ReferendaMigrations<Runtime>(PhantomData<Runtime>);

impl<Runtime> GetMigrations for ReferendaMigrations<Runtime>
where
	Runtime: pallet_referenda::Config<Hash = PreimageHash>,
{
	fn get_migrations() -> Vec<Box<dyn Migration>> {
		let pallet_referenda_migrate_v0_to_v1 =
			PalletReferendaMigrateV0ToV1::<Runtime>(Default::default());
		vec![Box::new(pallet_referenda_migrate_v0_to_v1)]
	}
}

pub struct CommonMigrations<Runtime, Council, Tech, Treasury, OpenTech>(
	PhantomData<(Runtime, Council, Tech, Treasury, OpenTech)>,
);

impl<Runtime, Council, Tech, Treasury, OpenTech> GetMigrations
	for CommonMigrations<Runtime, Council, Tech, Treasury, OpenTech>
where
	Runtime: pallet_author_mapping::Config,
	Runtime: pallet_parachain_staking::Config,
	Runtime: pallet_scheduler::Config<Hash = PreimageHash>,
	Runtime: AuthorSlotFilterConfig,
	Council: GetStorageVersion + PalletInfoAccess + 'static,
	Tech: GetStorageVersion + PalletInfoAccess + 'static,
	Treasury: GetStorageVersion + PalletInfoAccess + 'static,
	OpenTech: GetStorageVersion + PalletInfoAccess + 'static,
	Runtime: pallet_democracy::Config<Hash = PreimageHash>,
	Runtime: pallet_preimage::Config<Hash = PreimageHash>,
	Runtime: pallet_asset_manager::Config,
	<Runtime as pallet_asset_manager::Config>::ForeignAssetType: From<xcm::v3::MultiLocation>,
	Runtime: pallet_xcm_transactor::Config,
	Runtime: pallet_moonbeam_orbiters::Config,
	Runtime: cumulus_pallet_parachain_system::Config,
	u32: From<<<<Runtime as frame_system::Config>::Block as Block>::Header as Header>::Number>,
	Runtime: pallet_balances::Config,
	Runtime: pallet_referenda::Config,
	Runtime::AccountId: Default,
{
	fn get_migrations() -> Vec<Box<dyn Migration>> {
		// let migration_author_mapping_twox_to_blake = AuthorMappingTwoXToBlake::<Runtime> {
		// 	0: Default::default(),
		// };

		// let migration_parachain_staking_purge_stale_storage =
		// 	ParachainStakingPurgeStaleStorage::<Runtime>(Default::default());
		// let migration_parachain_staking_manual_exits =
		// 	ParachainStakingManualExits::<Runtime>(Default::default());
		// let migration_parachain_staking_increase_max_delegations_per_candidate =
		// 	ParachainStakingIncreaseMaxDelegationsPerCandidate::<Runtime>(Default::default());
		// let migration_parachain_staking_split_candidate_state =
		// 	ParachainStakingSplitCandidateState::<Runtime>(Default::default());
		// let migration_parachain_staking_patch_incorrect_delegation_sums =
		//	ParachainStakingPatchIncorrectDelegationSums::<Runtime>(Default::default());

		// let migration_scheduler_v3 = SchedulerMigrationV3::<Runtime>(Default::default());

		// let migration_base_fee = MigrateBaseFeePerGas::<Runtime>(Default::default());

		// TODO: this is a lot of allocation to do upon every get() call. this *should* be avoided
		// except when pallet_migrations undergoes a runtime upgrade -- but TODO: review

		// let migration_author_slot_filter_eligible_ratio_to_eligibility_count =
		// 	AuthorSlotFilterEligibleRatioToEligiblityCount::<Runtime>(Default::default());
		// let migration_author_mapping_add_keys_to_registration_info =
		// 	AuthorMappingAddKeysToRegistrationInfo::<Runtime>(Default::default());
		// let staking_delegator_state_requests =
		// 	ParachainStakingSplitDelegatorStateIntoDelegationScheduledRequests::<Runtime>(
		// 		Default::default(),
		// 	);
		// let migration_author_mapping_add_account_id_to_nimbus_lookup =
		//	AuthorMappingAddAccountIdToNimbusLookup::<Runtime>(Default::default());

		// let xcm_transactor_max_weight =
		// 	XcmTransactorMaxTransactWeight::<Runtime>(Default::default());

		// let asset_manager_units_with_asset_type =
		// 	AssetManagerUnitsWithAssetType::<Runtime>(Default::default());

		// let asset_manager_populate_asset_type_id_storage =
		// 	AssetManagerPopulateAssetTypeIdStorage::<Runtime>(Default::default());

		// let asset_manager_change_statemine_prefixes = AssetManagerChangeStateminePrefixes::<
		// 	Runtime,
		// 	StatemineParaIdInfo,
		// 	StatemineAssetsInstanceInfo,
		// >(Default::default());

		// let xcm_supported_assets = XcmPaymentSupportedAssets::<Runtime>(Default::default());

		// let migration_elasticity = MigrateBaseFeeElasticity::<Runtime>(Default::default());
		//let staking_at_stake_auto_compound =
		//	ParachainStakingMigrateAtStakeAutoCompound::<Runtime>(Default::default());

		//let scheduler_to_v4 = SchedulerMigrationV4::<Runtime>(Default::default());
		//let democracy_migration_hash_to_bounded_call =
		//	DemocracryMigrationHashToBoundedCall::<Runtime>(Default::default());
		//let preimage_migration_hash_to_bounded_call =
		//	PreimageMigrationHashToBoundedCall::<Runtime>(Default::default());
		//let asset_manager_to_xcm_v3 =
		//	PalletAssetManagerMigrateXcmV2ToV3::<Runtime>(Default::default());
		//let xcm_transactor_to_xcm_v3 =
		//	PalletXcmTransactorMigrateXcmV2ToV3::<Runtime>(Default::default());
		//let remove_min_bond_for_old_orbiter_collators =
		//	RemoveMinBondForOrbiterCollators::<Runtime>(Default::default());
		let update_first_round_relay_block_number =
			UpdateFirstRoundRelayBlockNumber::<Runtime>(Default::default());
		let missing_balances_migrations = MissingBalancesMigrations::<Runtime>(Default::default());
		let fix_pallet_versions =
			FixIncorrectPalletVersions::<Runtime, Treasury, OpenTech>(Default::default());

		vec![
			// completed in runtime 800
			// Box::new(migration_author_mapping_twox_to_blake),
			// completed in runtime 900
			// completed in runtime 1000
			// Box::new(migration_parachain_staking_purge_stale_storage),
			// completed in runtime 1000
			// Box::new(migration_parachain_staking_manual_exits),
			// completed in runtime 1101
			// Box::new(migration_parachain_staking_increase_max_delegations_per_candidate),
			// completed in runtime 1201
			// Box::new(migration_parachain_staking_split_candidate_state),
			// completed in runtime 1201
			// Box::new(xcm_transactor_max_weight),
			// completed in runtime 1201
			// Box::new(asset_manager_units_with_asset_type),
			// completed in runtime 1201
			// Box::new(asset_manager_change_statemine_prefixes),
			// completed in runtime 1201
			// Box::new(asset_manager_populate_asset_type_id_storage),
			// completed in runtime 1300
			// Box::new(migration_scheduler_v3),
			// completed in runtime 1300
			// Box::new(migration_parachain_staking_patch_incorrect_delegation_sums),
			// completed in runtime 1300
			// Box::new(migration_base_fee),
			// completed in runtime 1300
			// Box::new(xcm_supported_assets),
			// completed in runtime 1500
			// Box::new(migration_author_slot_filter_eligible_ratio_to_eligibility_count),
			// Box::new(migration_author_mapping_add_keys_to_registration_info),
			// Box::new(staking_delegator_state_requests),
			// completed in runtime 1600
			// Box::new(migration_author_mapping_add_account_id_to_nimbus_lookup),
			// completed in runtime 1600
			// Box::new(xcm_transactor_transact_signed),
			// completed in runtime 1700
			//Box::new(migration_elasticity),
			// completed in runtime 1900
			//Box::new(staking_at_stake_auto_compound),
			// completed in runtime 2000
			//Box::new(scheduler_to_v4),
			//Box::new(democracy_migration_hash_to_bounded_call),
			//Box::new(preimage_migration_hash_to_bounded_call),
			//Box::new(asset_manager_to_xcm_v3),
			//Box::new(xcm_transactor_to_xcm_v3),
			Box::new(update_first_round_relay_block_number),
			//Box::new(remove_min_bond_for_old_orbiter_collators),
			Box::new(missing_balances_migrations),
			Box::new(fix_pallet_versions),
		]
	}
}

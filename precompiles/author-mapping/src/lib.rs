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

//! Precompile to interact with pallet author mapping through an evm precompile.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(assert_matches)]

use fp_evm::PrecompileHandle;
use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
	traits::Get,
};
use pallet_author_mapping::Call as AuthorMappingCall;
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_core::crypto::UncheckedFrom;
use sp_core::H256;
use sp_std::marker::PhantomData;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// A precompile to wrap the functionality from pallet author mapping.
pub struct AuthorMappingPrecompile<Runtime>(PhantomData<Runtime>);

/// Bound for the keys size.
/// Pallet will check that the size exactly matches, but we want to bound the parser to
/// not accept larger bytes.
pub struct GetKeysSize<R>(PhantomData<R>);

impl<R: pallet_author_mapping::Config> Get<u32> for GetKeysSize<R> {
	fn get() -> u32 {
		pallet_author_mapping::pallet::keys_size::<R>()
			.try_into()
			.expect("keys size to fit in u32")
	}
}

#[precompile_utils::precompile]
#[precompile::test_concrete_types(mock::Runtime)]
impl<Runtime> AuthorMappingPrecompile<Runtime>
where
	Runtime: pallet_author_mapping::Config + pallet_evm::Config + frame_system::Config,
	Runtime::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::Call as Dispatchable>::Origin: From<Option<Runtime::AccountId>>,
	Runtime::Call: From<AuthorMappingCall<Runtime>>,
	Runtime::Hash: From<H256>,
{
	// The dispatchable wrappers are next. They dispatch a Substrate inner Call.
	#[precompile::public("addAssociation(bytes32)")]
	#[precompile::public("add_association(bytes32)")]
	fn add_association(handle: &mut impl PrecompileHandle, nimbus_id: H256) -> EvmResult {
		let nimbus_id = sp_core::sr25519::Public::unchecked_from(nimbus_id).into();

		log::trace!(
			target: "author-mapping-precompile",
			"Associating author id {:?}", nimbus_id
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = AuthorMappingCall::<Runtime>::add_association { nimbus_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("updateAssociation(bytes32,bytes32)")]
	#[precompile::public("update_association(bytes32,bytes32)")]
	fn update_association(
		handle: &mut impl PrecompileHandle,
		old_nimbus_id: H256,
		new_nimbus_id: H256,
	) -> EvmResult {
		let old_nimbus_id = sp_core::sr25519::Public::unchecked_from(old_nimbus_id).into();
		let new_nimbus_id = sp_core::sr25519::Public::unchecked_from(new_nimbus_id).into();

		log::trace!(
			target: "author-mapping-precompile",
			"Updating author id {:?} for {:?}", old_nimbus_id, new_nimbus_id
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = AuthorMappingCall::<Runtime>::update_association {
			old_nimbus_id,
			new_nimbus_id,
		};

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("clearAssociation(bytes32)")]
	#[precompile::public("clear_association(bytes32)")]
	fn clear_association(handle: &mut impl PrecompileHandle, nimbus_id: H256) -> EvmResult {
		let nimbus_id = sp_core::sr25519::Public::unchecked_from(nimbus_id).into();

		log::trace!(
			target: "author-mapping-precompile",
			"Clearing author id {:?}", nimbus_id
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = AuthorMappingCall::<Runtime>::clear_association { nimbus_id };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("removeKeys()")]
	#[precompile::public("remove_keys()")]
	fn remove_keys(handle: &mut impl PrecompileHandle) -> EvmResult {
		log::trace!(
			target: "author-mapping-precompile",
			"Removing keys"
		);

		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = AuthorMappingCall::<Runtime>::remove_keys {};

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("setKeys(bytes)")]
	#[precompile::public("set_keys(bytes)")]
	fn set_keys(
		handle: &mut impl PrecompileHandle,
		keys: BoundedBytes<GetKeysSize<Runtime>>,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = AuthorMappingCall::<Runtime>::set_keys { keys: keys.into() };

		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}

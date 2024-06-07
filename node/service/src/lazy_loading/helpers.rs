// Copyright 2024 Moonbeam foundation
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

use crate::lazy_loading;
use cumulus_primitives_core::BlockT;
use sc_client_api::{Backend, BlockImportOperation, NewBlockState};
use sp_runtime::traits::{Header, One};
use sp_runtime::Saturating;
use sp_storage::{StateVersion, Storage};
use std::sync::Arc;

pub fn produce_genesis_block<TBl: BlockT + sp_runtime::DeserializeOwned>(
	backend: Arc<lazy_loading::backend::Backend<TBl>>,
) -> sp_blockchain::Result<()> {
	let mut op = backend.begin_operation()?;
	op.before_fork = true;

	let genesis_block_hash: TBl::Hash = backend
		.rpc_client
		.block_hash::<TBl>(Some(0))
		.unwrap()
		.unwrap();

	let genesis_block: TBl = backend
		.rpc_client
		.block::<TBl, _>(Some(genesis_block_hash))
		.unwrap()
		.unwrap();

	let _ = op.set_block_data(
		genesis_block.header().clone(),
		Some(genesis_block.extrinsics().to_vec()),
		None,
		None,
		NewBlockState::Final,
	);

	backend.commit_operation(op)
}

pub fn produce_first_block<TBl: BlockT + sp_runtime::DeserializeOwned>(
	backend: Arc<lazy_loading::backend::Backend<TBl>>,
	last_block: TBl,
	state_overrides: Vec<(Vec<u8>, Vec<u8>)>,
) -> sp_blockchain::Result<()> {
	use sc_client_api::HeaderBackend;
	let mut op = backend.begin_operation()?;

	let state_root = op.reset_storage(
		Storage {
			top: state_overrides.into_iter().collect(),
			children_default: Default::default(),
		},
		StateVersion::V1,
	)?;

	let head_info = backend.blockchain.info();
	let next_block_number = head_info.finalized_number.saturating_add(One::one());

	let header: TBl::Header = TBl::Header::new(
		next_block_number,
		last_block.header().extrinsics_root().clone(),
		state_root,
		head_info.finalized_hash,
		Default::default(),
	);

	let _ = op.set_block_data(
		header.clone(),
		Some(last_block.extrinsics().to_vec()),
		None,
		None,
		NewBlockState::Final,
	);

	backend.commit_operation(op)
}

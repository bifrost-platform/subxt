// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Module with configuration which reflects BridgeHubWococo runtime setup
//! (AccountId, Headers, Hashes...)

#![cfg_attr(not(feature = "std"), no_std)]

pub use bp_bridge_hub_cumulus::*;
use bp_messages::*;
use bp_runtime::{
	decl_bridge_finality_runtime_apis, decl_bridge_messages_runtime_apis, Chain, Parachain,
};
use frame_support::dispatch::DispatchClass;
use sp_runtime::RuntimeDebug;

/// BridgeHubWococo parachain.
#[derive(RuntimeDebug)]
pub struct BridgeHubWococo;

impl Chain for BridgeHubWococo {
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hasher = Hasher;
	type Header = Header;

	type AccountId = AccountId;
	type Balance = Balance;
	type Nonce = Nonce;
	type Signature = Signature;

	fn max_extrinsic_size() -> u32 {
		*BlockLength::get().max.get(DispatchClass::Normal)
	}

	fn max_extrinsic_weight() -> Weight {
		BlockWeights::get()
			.get(DispatchClass::Normal)
			.max_extrinsic
			.unwrap_or(Weight::MAX)
	}
}

impl Parachain for BridgeHubWococo {
	const PARACHAIN_ID: u32 = BRIDGE_HUB_WOCOCO_PARACHAIN_ID;
}

/// Identifier of BridgeHubWococo in the Wococo relay chain.
pub const BRIDGE_HUB_WOCOCO_PARACHAIN_ID: u32 = 1014;

/// Name of the With-BridgeHubWococo messages pallet instance that is deployed at bridged chains.
pub const WITH_BRIDGE_HUB_WOCOCO_MESSAGES_PALLET_NAME: &str = "BridgeWococoMessages";

/// Name of the With-BridgeHubWococo bridge-relayers pallet instance that is deployed at bridged
/// chains.
pub const WITH_BRIDGE_HUB_WOCOCO_RELAYERS_PALLET_NAME: &str = "BridgeRelayers";

/// Pallet index of `BridgeRococoMessages: pallet_bridge_messages::<Instance2>`.
pub const WITH_BRIDGE_WOCOCO_TO_ROCOCO_MESSAGES_PALLET_INDEX: u8 = 45;

decl_bridge_finality_runtime_apis!(bridge_hub_wococo);
decl_bridge_messages_runtime_apis!(bridge_hub_wococo);

frame_support::parameter_types! {
	/// The XCM fee that is paid for executing XCM program (with `ExportMessage` instruction) at the Wococo
	/// BridgeHub.
	/// (initially was calculated by test `BridgeHubWococo::can_calculate_weight_for_paid_export_message_with_reserve_transfer` + `33%`)
	pub const BridgeHubWococoBaseXcmFeeInWocs: u128 = 1624803349;

	/// Transaction fee that is paid at the Wococo BridgeHub for delivering single inbound message.
	/// (initially was calculated by test `BridgeHubWococo::can_calculate_fee_for_complex_message_delivery_transaction` + `33%`)
	pub const BridgeHubWococoBaseDeliveryFeeInWocs: u128 = 6417262881;

	/// Transaction fee that is paid at the Wococo BridgeHub for delivering single outbound message confirmation.
	/// (initially was calculated by test `BridgeHubWococo::can_calculate_fee_for_complex_message_confirmation_transaction` + `33%`)
	pub const BridgeHubWococoBaseConfirmationFeeInWocs: u128 = 6159996668;
}
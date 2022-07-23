use std::collections::HashSet;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, AccountId, Balance, Gas};
use uint::construct_uint;
use crate::errors::*;

/// Attach no deposit.
pub const NO_DEPOSIT: u128 = 0;

/// 10T gas for basic operation
pub const GAS_FOR_BASIC_OP: Gas = 10_000_000_000_000;

/// hotfix_insuffient_gas_for_mft_resolve_transfer.
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = 20_000_000_000_000;

pub const GAS_FOR_FT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;

/// Amount of gas for fungible token transfers, increased to 20T to support AS token contracts.
pub const GAS_FOR_FT_TRANSFER: Gas = 20_000_000_000_000;

/// Fee divisor, allowing to provide fee in bps.
pub const FEE_DIVISOR: u32 = 10_000;

/// Initial shares supply on deposit of liquidity.
pub const INIT_SHARES_SUPPLY: u128 = 1_000_000_000_000_000_000_000_000;
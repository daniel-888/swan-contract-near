use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::*;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env,
    ext_contract,
    json_types::U128,
    log,
    near_bindgen,
    AccountId,
    PanicOnDefault,
    Promise,
    PromiseOrValue,
    BorshStorageKey,
    Balance
};
use near_contract_standards::fungible_token::{FungibleToken};
use near_contract_standards::fungible_token::core_impl::ext_fungible_token;

use crate::action::{Action, ActionResult};
use crate::errors::*;
use crate::utils::*;

mod token_receiver;
mod errors;
mod action;
mod utils;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Infos,
}

#[derive(BorshDeserialize, BorshSerialize, Default, Clone)]
pub struct Info {
    pub deposit_amount: Balance,
    pub preinformed_amount: Balance,
    pub last_preinformed_time: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    principal_token_id: AccountId,
    target_token_id: AccountId,
    epoch_duration: u64,
    epoch_start: u64,
    deposite_infos: LookupMap<AccountId, Info>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        principal_token_id: ValidAccountId, 
        target_token_id: ValidAccountId,
        epoch_duration: u64,
        epoch_start: u64,
    ) -> Self {
        Self { 
            principal_token_id: principal_token_id.as_ref().clone(), 
            target_token_id: target_token_id.as_ref().clone(),
            epoch_duration: epoch_duration.clone(),
            epoch_start: epoch_start.clone(),
            deposite_infos: LookupMap::new(StorageKey::Infos)
        }
    }

    pub fn preinform(&mut self, amount: Balance) {
        self.assert_preinformable();
        let sender_id = env::predecessor_account_id();
        let info = self.internal_unwrap_info(&sender_id);
        assert!(amount <= info.deposit_amount, "{}", errors::ERR13_AMOUNT_EXCEED);
        let new_info = Info {
            deposit_amount: info.deposit_amount,
            preinformed_amount: info.preinformed_amount + amount,
            last_preinformed_time: env::block_timestamp(),
        };
        self.internal_save_info(&sender_id, new_info);
    }

    pub fn withdraw(&mut self, token_id: AccountId, amount: Balance) {
        let sender_id = env::predecessor_account_id();
        let info = self.internal_unwrap_info(&sender_id);
        assert!(amount <= info.preinformed_amount, "{}", errors::ERR12_NOT_ENOUGH_PREINFORM);
        ext_fungible_token::ft_transfer(
            sender_id.clone(),
            U128(amount),
            None,
            &token_id,
            1,
            GAS_FOR_FT_TRANSFER,
        );
        let new_info = Info {
            deposit_amount: info.deposit_amount - amount,
            preinformed_amount: info.preinformed_amount - amount,
            last_preinformed_time: env::block_timestamp(),
        };
        self.internal_save_info(&sender_id, new_info);
    }

    pub fn trade(&mut self, receiver_id: AccountId, token_id: AccountId, amount: Balance, msg: String) {
        ext_fungible_token::ft_transfer_call(
            receiver_id,
            U128(amount),
            None,
            msg,
            &token_id,
            1,
            GAS_FOR_FT_TRANSFER_CALL
        );
    }

}

impl Contract {
    pub fn internal_deposit(
        &mut self, 
        sender_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
    ) {
        assert_eq!(token_id, &self.principal_token_id);
        let deposit_val = self.internal_get_info(sender_id);
        match deposit_val {
            Some(val) => {
                let new_val = val.deposit_amount + amount;
                let new_info = Info {
                    deposit_amount: new_val,
                    preinformed_amount: val.preinformed_amount,
                    last_preinformed_time: val.last_preinformed_time,
                };
                self.internal_save_info(sender_id, new_info);
            },
            None => {
                let new_info = Info {
                    deposit_amount: amount,
                    preinformed_amount: 0,
                    last_preinformed_time: self.epoch_start,
                };
                self.internal_save_info(sender_id, new_info);
            }
        }
    }
}

impl Contract {
    pub fn internal_get_info(&self, account_id: &AccountId) -> Option<Info> {
        self.deposite_infos.get(account_id).map(|b| b)
    }

    pub fn internal_unwrap_info(&self, account_id: &AccountId) -> Info {
        self.internal_get_info(account_id).expect(errors::ERR10_ACC_NOT_REGISTERED)
    }

    pub fn internal_save_info(&mut self, account_id: &AccountId, info: Info) {
        self.deposite_infos.insert(&account_id, &info);
    }
}

impl Contract {
    pub fn assert_preinformable(&self) {
        let current_time = env::block_timestamp();
        let to_next_epoch = (current_time
            .checked_sub(self.epoch_start)
            .unwrap() / self.epoch_duration + 1)
            .checked_mul(self.epoch_duration)
            .unwrap().checked_add(self.epoch_start)
            .unwrap().checked_sub(current_time).unwrap();
        assert!(to_next_epoch >= 86400 * 3, "{}", errors::ERR11_NOT_TIME_FOR_PREINFORM);
    }
}
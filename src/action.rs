use crate::errors::ERR41_WRONG_ACTION_RESULT;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, json_types::U128, AccountId, Balance};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DepositAction {
  pub amount: Balance,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum Action {
  Deposit(DepositAction),
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ActionResult {
  None,
  Amount(U128),
}

impl ActionResult {
  pub fn to_amount(self) -> Balance {
    match self {
      ActionResult::Amount(result) => result.0,
      _ => env::panic(ERR41_WRONG_ACTION_RESULT.as_bytes()),
    }
  }
}
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseOrValue};

use crate::*;

pub const VIRTUAL_ACC: &str = "@";

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
enum TokenReceiverMessage {
  Execute {
    action: Action,
  },
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
  #[allow(unreachable_code)]
  fn ft_on_transfer(
    &mut self, 
    sender_id: ValidAccountId, 
    amount: U128, 
    msg: String
  ) -> PromiseOrValue<U128> {
    let token_in = env::predecessor_account_id();
    let message = serde_json::from_str::<TokenReceiverMessage>(&msg).expect(ERR28_WRONG_MSG_FORMAT);
    match message {
      TokenReceiverMessage::Execute {
        action
      } => {
        match action {
          Action::Deposit(depositAction) => {
            self.internal_deposit(sender_id.as_ref(), &token_in, amount.into());
            PromiseOrValue::Value(U128(0))
          }
        }
      }
    }
  }
}
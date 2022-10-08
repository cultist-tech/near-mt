use near_sdk::{AccountId, PromiseOrValue, env, near_bindgen};
use near_sdk::json_types::U128;
use mfight_sdk::ft::FungibleTokenReceiver;
use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
  fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, _msg: String) -> PromiseOrValue<U128> {
    let ft_token_id = env::predecessor_account_id();
    let signer_id = env::signer_account_id();

    assert_eq!(
      &sender_id,
      &signer_id,
      "owner_id should be signer_id"
    );
    assert_ne!(
      ft_token_id,
      signer_id,
      "nft_on_approve should only be called via cross-contract call"
    );

    self.mft.internal_on_ft_transfer(&ft_token_id, &sender_id, &amount);

    PromiseOrValue::Value(U128::from(0))
  }
}

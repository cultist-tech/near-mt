use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue, BorshStorageKey, StorageUsage};
use near_sdk::collections::{TreeMap};
use mfight_sdk::pause::PauseFeature;
use mfight_sdk::owner::OwnerFeature;
use mfight_sdk::blacklist::BlacklistFeature;
use mfight_sdk::mt::MultiFungibleToken;
use mfight_sdk::mt::base::metadata::MtToken;

mod ft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pause: PauseFeature,
  owner: OwnerFeature,
  blacklist: BlacklistFeature,
  mft: MultiFungibleToken,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  BlacklistAccounts,
  BindToOwnerTokens,

  MtAccounts,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id,
    )
  }

  #[init]
  pub fn new(owner_id: AccountId) -> Self {
    let this = Self {
      mft: MultiFungibleToken::new(StorageKey::MtAccounts),
      pause: PauseFeature::new(),
      owner: OwnerFeature::new(owner_id.clone()),
      blacklist: BlacklistFeature::new(StorageKey::BlacklistAccounts),
    };

    this
  }

  fn assert_owner(&self) {
    self.owner.assert_owner();
  }

  fn assert_transfer(&self) {}

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldMt {
      pub tokens: TreeMap<AccountId, MtToken>,

      pub account_storage_usage: StorageUsage,
    }

    #[derive(BorshDeserialize)]
    struct Old {
      pause: PauseFeature,
      owner: OwnerFeature,
      blacklist: BlacklistFeature,
      mft: OldMt,
    }

    let old: Old = env::state_read().expect("Error");

    let mt = MultiFungibleToken {
      tokens: old.mft.tokens,
      account_storage_usage: old.mft.account_storage_usage,
    };

    Self {
      mft: mt,
      pause: old.pause,
      blacklist: old.blacklist,
      owner: old.owner,
    }
  }
}

mfight_sdk::impl_pause_feature!(Contract, pause, assert_owner);
mfight_sdk::impl_owner_feature!(Contract, owner);
mfight_sdk::impl_blacklist_feature!(Contract, blacklist, assert_owner);

mfight_sdk::impl_multi_fungible_token_core!(Contract, mft, assert_owner, assert_transfer);

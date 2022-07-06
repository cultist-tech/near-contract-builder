use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault,
    Promise, PromiseOrValue, BorshStorageKey,
};
use crate::pause::{PauseFeature};
use crate::owner::OwnerFeature;
use crate::blacklist::BlacklistFeature;
use crate::royalty::RoyaltyFeature;
use crate::bind_to_owner::BindToOwnerFeature;

mod pause;
mod owner;
mod blacklist;
mod royalty;
mod bind_to_owner;
mod event;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pause: PauseFeature,
  owner: OwnerFeature,
  blacklist: BlacklistFeature,
  royalty: RoyaltyFeature,
  bind_to_owner: BindToOwnerFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  BlacklistAccounts,
  RoyaltyByToken,
  BindToOwnerTokens,
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
      pause: PauseFeature::new(),
      owner: OwnerFeature::new(owner_id.clone()),
      blacklist: BlacklistFeature::new(StorageKey::BlacklistAccounts),
      royalty: RoyaltyFeature::new(owner_id.clone(), 2000, StorageKey::RoyaltyByToken),
      bind_to_owner: BindToOwnerFeature::new(StorageKey::BindToOwnerTokens),
    };

    this
  }

  fn assert_owner(&self) {
    self.owner.assert_owner();
  }
}

impl_pause_feature!(Contract, pause, assert_owner);
impl_owner_feature!(Contract, owner, assert_owner);
impl_blacklist_feature!(Contract, blacklist, assert_owner);
impl_royalty_feature!(Contract, royalty, assert_owner);

use near_sdk::{AccountId, env};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::owner::ContractOwner;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OwnerFeature {
  owner_id: AccountId,
}

impl OwnerFeature {
  pub fn new(owner_id: AccountId) -> Self {
    let this = Self {
      owner_id
    };

    this
  }

  pub(crate) fn internal_set_owner(&mut self, account_id: &AccountId) -> AccountId {
    self.owner_id = account_id.clone();

    self.get_owner()
  }

  pub(crate) fn assert_owner(&self) {
    if self.owner_id != env::predecessor_account_id() {
      env::panic_str("Access Denied");
    }
  }
}

impl ContractOwner for OwnerFeature {
  fn get_owner(&self) -> AccountId {
    self.owner_id.clone()
  }

  fn set_owner(&mut self, account_id: AccountId) -> AccountId {
    self.internal_set_owner(&account_id)
  }
}

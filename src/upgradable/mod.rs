pub use upgradable_impl::*;
use near_sdk::AccountId;
use crate::nft::TokenId;

pub mod upgradable_impl;

pub trait UpgradableMethods {
  fn upgrade_token(&mut self, token_id: TokenId);
}

use crate::nft::burn::{NonFungibleTokenBurn};
use near_sdk::{env};
use crate::nft::base::NonFungibleToken;
use crate::nft::metadata::TokenId;

impl NonFungibleTokenBurn for NonFungibleToken {
  fn nft_burn(&mut self, token_id: TokenId) {
    let sender_id = env::predecessor_account_id();

    self.internal_burn_token(&sender_id, &token_id);
  }
}

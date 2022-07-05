#[macro_export]
macro_rules! impl_non_fungible_token_payout {
    ($contract: ident, $instance: ident, $nft: ident, $royalty: ident) => {
        use $crate::nft_payout::{NonFungibleTokenPayout, Payout};

        #[near_bindgen]
        impl NonFungibleTokenPayout for $contract {
          #[payable]
          fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: String, approval_id: u64, balance: U128, max_len_payout: u32, memo: Option<String>) -> Payout {
              self.$instance.internal_transfer_payout(&mut self.$nft, &self.$royalty, receiver_id, token_id, approval_id, balance, max_len_payout, memo)
          }

          fn nft_payout(&self, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
              self.$instance.internal_payout(&self.$nft, &self.$royalty, token_id, balance, max_len_payout)
          }
        }
    };
}

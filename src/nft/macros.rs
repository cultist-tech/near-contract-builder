// Core

#[macro_export]
macro_rules! impl_non_fungible_token_core {
    ($contract: ident, $token: ident, $assert_transfer: ident) => {
        use $crate::nft::base::NonFungibleTokenCore;
        use $crate::nft::base::NonFungibleTokenResolver;

        #[near_bindgen]
        impl NonFungibleTokenCore for $contract {
            #[payable]
            fn nft_transfer(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
            ) {
                self.$assert_transfer(&token_id);
                self.$token.nft_transfer(receiver_id, token_id, approval_id, memo)
            }

            #[payable]
            fn nft_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                approval_id: Option<u64>,
                memo: Option<String>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                self.$assert_transfer(&token_id);
                self.$token.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
            }

            fn nft_token(&self, token_id: TokenId) -> Option<Token> {
                self.$token.nft_token(token_id)
            }
        }

        #[near_bindgen]
        impl NonFungibleTokenResolver for $contract {
            #[private]
            fn nft_resolve_transfer(
                &mut self,
                previous_owner_id: AccountId,
                receiver_id: AccountId,
                token_id: TokenId,
                approved_account_ids: Option<std::collections::HashMap<AccountId, u64>>,
            ) -> bool {
                self.$token.nft_resolve_transfer(
                    previous_owner_id,
                    receiver_id,
                    token_id,
                    approved_account_ids,
                )
            }
        }
    };
}

// Approval

#[macro_export]
macro_rules! impl_non_fungible_token_approval {
    ($contract: ident, $token: ident, $assert_approve: ident) => {
        use $crate::nft::NonFungibleTokenApproval;

        #[near_bindgen]
        impl NonFungibleTokenApproval for $contract {
            #[payable]
            fn nft_approve(
                &mut self,
                token_id: TokenId,
                account_id: AccountId,
                msg: Option<String>,
            ) -> Option<Promise> {
                self.$assert_approve(&token_id);
                self.$token.nft_approve(token_id, account_id, msg)
            }

            #[payable]
            fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
                self.$token.nft_revoke(token_id, account_id)
            }

            #[payable]
            fn nft_revoke_all(&mut self, token_id: TokenId) {
                self.$token.nft_revoke_all(token_id)
            }

            fn nft_is_approved(
                &self,
                token_id: TokenId,
                approved_account_id: AccountId,
                approval_id: Option<u64>,
            ) -> bool {
                self.$token.nft_is_approved(token_id, approved_account_id, approval_id)
            }
        }
    };
}

// Enumeration

#[macro_export]
macro_rules! impl_non_fungible_token_enumeration {
    ($contract: ident, $token: ident, $get_token: ident) => {
        use $crate::nft::{NonFungibleTokenEnumeration};

        #[near_bindgen]
        impl NonFungibleTokenEnumeration for $contract {
            fn nft_total_supply(&self) -> near_sdk::json_types::U128 {
                self.$token.nft_total_supply()
            }

            fn nft_tokens(
                &self,
                from_index: Option<near_sdk::json_types::U128>,
                limit: Option<u64>,
            ) -> Vec<Token> {
                self.$token.nft_tokens(from_index, limit)
            }

            fn nft_supply_for_owner(&self, account_id: AccountId) -> near_sdk::json_types::U128 {
                self.$token.nft_supply_for_owner(account_id)
            }

            fn nft_tokens_for_owner(
                &self,
                account_id: AccountId,
                from_index: Option<near_sdk::json_types::U128>,
                limit: Option<u64>,
            ) -> Vec<Token> {
                self.$token.nft_tokens_for_owner(account_id, from_index, limit)
            }

            fn nft_tokens_by_ids(
                &self,
                ids: Vec<TokenId>,
            ) -> Vec<Token> {
                self.$token.nft_tokens_by_ids(ids)
            }
        }
    };
}


#[macro_export]
macro_rules! impl_non_fungible_token_burn {
    ($contract: ident, $tokens: ident, $assert_burn: ident) => {
        use $crate::nft::{NonFungibleTokenBurn, TokenId};

        #[near_bindgen]
        impl NonFungibleTokenBurn for $contract {
          #[payable]
          fn nft_burn(&mut self, token_id: TokenId) {
            self.$assert_burn(&token_id);
            self.$tokens.nft_burn(token_id)
          }
        }
    };
}


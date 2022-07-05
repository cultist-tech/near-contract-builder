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
use crate::nft_payout::NftPayoutFeature;
use crate::bind_to_owner::BindToOwnerFeature;
use crate::nft::base::NonFungibleToken;
use crate::nft::Token;

mod pause;
mod owner;
mod blacklist;
mod royalty;
mod nft_payout;
mod bind_to_owner;
mod nft;
mod event;
// mod upgradable;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pause: PauseFeature,
  owner: OwnerFeature,
  blacklist: BlacklistFeature,
  royalty: RoyaltyFeature,
  nft_payout: NftPayoutFeature,
  nft: NonFungibleToken,
  bind_to_owner: BindToOwnerFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
  BlacklistAccounts,
  RoyaltyByToken,
  BindToOwnerTokens,

  NftTokenOwners,
  NftTokenMetadata,
  NftTokenRarity,
  NftTokenCollection,
  NftTokenType,
  NftTokenSubType,
  NftTokensForOwner,
  NftTokenApprovals,
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
      nft: NonFungibleToken::new(
        StorageKey::NftTokenOwners,
        Some(StorageKey::NftTokenMetadata),
        Some(StorageKey::NftTokensForOwner),
        Some(StorageKey::NftTokenApprovals),
        Some(StorageKey::NftTokenRarity),
        Some(StorageKey::NftTokenCollection),
        Some(StorageKey::NftTokenType),
        Some(StorageKey::NftTokenSubType),
      ),
      nft_payout: NftPayoutFeature::new(),
      bind_to_owner: BindToOwnerFeature::new(StorageKey::BindToOwnerTokens),
    };

    this
  }

  pub fn test(&self) -> u128 {
    self.pause.is_paused();

    128
  }

  fn assert_owner(&self) {
    self.owner.assert_owner();
  }

  fn assert_transfer(&self, token_id: &String) {
    let signer_id = env::predecessor_account_id();

    self.pause.assert_not_pause();
    self.blacklist.assert_not_blocked(&signer_id);
    self.bind_to_owner.assert_bind_to_player(&token_id);
  }

  fn assert_approve(&self, token_id: &String) {
    let signer_id = env::predecessor_account_id();

    self.pause.assert_not_pause();
    self.blacklist.assert_not_blocked(&signer_id);
    self.bind_to_owner.assert_bind_to_player(&token_id);
  }

  fn assert_burn(&self, token_id: &String) {
    let signer_id = env::predecessor_account_id();

    self.pause.assert_not_pause();
    self.blacklist.assert_not_blocked(&signer_id);
  }

  fn enum_get_token(&self, owner_id: AccountId, token_id: TokenId) -> Token {
    let metadata = self.nft.token_metadata_by_id.as_ref().unwrap().get(&token_id);
    let approved_account_ids =
      Some(self.nft.approvals_by_id.as_ref().unwrap().get(&token_id).unwrap_or_default());

    // custom
    // TODO update
    // let bind_to_owner = self.token_bind_by_id.as_ref().unwrap().get(&token_id);
    let rarity = self.nft.token_rarity_by_id.as_ref().unwrap().get(&token_id);
    // TODO update
    // let royalty = self.token_royalty_by_id.as_ref().unwrap().get(&token_id);
    let collection = self.nft.token_collection_by_id.as_ref().unwrap().get(&token_id);
    let token_type = self.nft.token_type_by_id.as_ref().unwrap().get(&token_id);
    let token_sub_type = self.nft.token_sub_type_by_id.as_ref().unwrap().get(&token_id);
    // TODO update
    // let fractionation_id = self.fractionation_token_by_id.as_ref().unwrap().get(&token_id);
    // TODO update
    // let sale_id = self.sale_by_token.as_ref().unwrap().get(&token_id);

    Token {
      token_id,
      reference_id: None,
      owner_id,
      metadata,
      approved_account_ids,
      sale_id: None,
      royalty: None,
      collection,
      token_type,
      token_sub_type,
      rarity,
      bind_to_owner: Some(false),
      fractionation_id: None,
    }
  }
}

impl_pause_feature!(Contract, pause, assert_owner);
impl_owner_feature!(Contract, owner, assert_owner);
impl_blacklist_feature!(Contract, blacklist, assert_owner);
impl_royalty_feature!(Contract, royalty, assert_owner);

impl_non_fungible_token_core!(Contract, nft, assert_transfer);
impl_non_fungible_token_enumeration!(Contract, nft, enum_get_token);
impl_non_fungible_token_approval!(Contract, nft, assert_approve);
impl_non_fungible_token_burn!(Contract, nft, assert_burn);
impl_non_fungible_token_payout!(Contract, nft_payout, nft, royalty);

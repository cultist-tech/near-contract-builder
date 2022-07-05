use near_sdk::json_types::U128;
use crate::royalty::{MINTER_ROYALTY_CAP, CONTRACT_ROYALTY_CAP, RoyaltyFeature};
use near_sdk::{env, assert_one_yocto, AccountId};
use std::collections::HashMap;
use near_sdk::serde::{Deserialize, Serialize};
use crate::nft_payout::NonFungibleTokenPayout;
use crate::royalty::{royalty_to_payout};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::nft::NonFungibleToken;
use crate::nft::utils::refund_approved_account_ids;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
  pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftPayoutFeature {}

impl NftPayoutFeature {
  pub(crate) fn new() -> Self {
    let this = Self {};

    this
  }

  pub(crate) fn internal_payout(&self, nft: &NonFungibleToken, royalty: &RoyaltyFeature, token_id: String, balance: U128, max_len_payout: u32) -> Payout {
    let token_royalty = royalty.token_royalty_by_id.get(&token_id);
    let owner_id = nft.owner_by_id.get(&token_id).expect("No token");

    // compute payouts based on balance option
    // adds in contract_royalty and computes previous owner royalty from remainder
    let mut total_perpetual = 0;
    let balance_u128 = u128::from(balance);
    let mut payout: Payout = Payout {
      payout: HashMap::new(),
    };

    if let Some(token_royalty) = token_royalty {
      assert!(token_royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

      for (k, v) in token_royalty.iter() {
        let key = k.clone();
        if key != owner_id {
          payout.payout.insert(key, royalty_to_payout(*v, balance_u128));
          total_perpetual += *v;
        }
      }
    }

    // payout to contract owner - may be previous token owner, they get remainder of balance
    if royalty.amount > 0 && royalty.receiver_id != owner_id {
      payout.payout.insert(royalty.receiver_id.clone(), royalty_to_payout(royalty.amount, balance_u128));
      total_perpetual += royalty.amount;
    }

    assert!(total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP, "Royalties should not be more than caps");
    // payout to previous owner
    payout.payout.insert(owner_id, royalty_to_payout(10000 - total_perpetual, balance_u128));

    payout
  }

  pub(crate) fn internal_transfer_payout(
    &mut self,
    nft: &mut NonFungibleToken,
    royalty: &RoyaltyFeature,
    receiver_id: AccountId,
    token_id: String,
    approval_id: u64,
    balance: U128,
    max_len_payout: u32,
    memo: Option<String>,
  ) -> Payout {
    assert_one_yocto();

    let sender_id = env::predecessor_account_id();
    let (owner_id, approved_account_ids) = nft.internal_transfer(
      &sender_id,
      &receiver_id,
      &token_id,
      Some(approval_id),
      memo,
    );

    if let Some(approved_account_ids) = approved_account_ids {
      refund_approved_account_ids(
        owner_id.clone(),
        &approved_account_ids,
      );
    }

    // compute payouts based on balance option
    // adds in contract_royalty and computes previous owner royalty from remainder
    let mut total_perpetual = 0;
    let balance_u128 = u128::from(balance);
    let mut payout: Payout = Payout { payout: HashMap::new() };
    let token_royalty = royalty.token_royalty_by_id.get(&token_id);

    if let Some(token_royalty) = token_royalty {
      assert!(token_royalty.len() as u32 <= max_len_payout, "Market cannot payout to that many receivers");

      for (k, v) in token_royalty.iter() {
        let key = k.clone();
        if key != owner_id {
          payout.payout.insert(key, royalty_to_payout(*v, balance_u128));
          total_perpetual += *v;
        }
      }
    }

    // payout to contract owner - may be previous token owner, they get remainder of balance
    if royalty.amount > 0 && royalty.receiver_id != owner_id {
      payout.payout.insert(royalty.receiver_id.clone(), royalty_to_payout(royalty.amount, balance_u128));
      total_perpetual += royalty.amount;
    }
    assert!(total_perpetual <= MINTER_ROYALTY_CAP + CONTRACT_ROYALTY_CAP, "Royalties should not be more than caps");
    // payout to previous owner
    payout.payout.insert(owner_id.clone(), royalty_to_payout(10000 - total_perpetual, balance_u128));

    // NftTransferPayout {
    //   token_id: &token_id,
    //   sender_id: &sender_id,
    //   receiver_id: &receiver_id,
    //   balance: &balance
    // }.emit();

    payout
  }
}

// impl NonFungibleTokenPayout for NftPayoutFeature {
//
// }

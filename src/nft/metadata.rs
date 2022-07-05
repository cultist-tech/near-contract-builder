use near_sdk::AccountId;
use std::collections::HashMap;
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

pub type TokenId = String;

/// Metadata on the individual token level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
  pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
  pub description: Option<String>, // free-form description
  pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
  pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
  pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
  pub issued_at: Option<String>, // ISO 8601 datetime when token was issued or minted
  pub expires_at: Option<String>, // ISO 8601 datetime when token expires
  pub starts_at: Option<String>, // ISO 8601 datetime when token starts being valid
  pub updated_at: Option<String>, // ISO 8601 datetime when token was last updated
  pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
  pub reference: Option<String>, // URL to an off-chain JSON file with more info.
  pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

/// In this implementation, the Token struct takes two extensions standards (metadata and approval) as optional fields, as they are frequently used in modern NFTs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
  pub token_id: TokenId,
  pub owner_id: AccountId,
  pub metadata: Option<TokenMetadata>,
  pub approved_account_ids: Option<HashMap<AccountId, u64>>,

  // custom
  pub reference_id: Option<TokenId>,

  pub sale_id: Option<TokenId>,
  pub royalty: Option<HashMap<AccountId, u32>>,

  pub collection: Option<TokenCollection>,
  pub token_type: Option<TokenType>,
  pub token_sub_type: Option<TokenSubType>,
  pub rarity: Option<TokenRarity>,

  pub bind_to_owner: Option<bool>,

  pub fractionation_id: Option<TokenId>,
}


// Custom

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenRarity {
  Common,
  Uncommon,
  Rare,
  Uniq,
  Epic,
  Legendary,
  Artefact
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenCollection {
  Fantasy,
  Medieval,
  Nordic,
  PostApoc,
  SteamPunk,
  Asian,
  CyberPunk,
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenType {
  Sketch,
  Badge,
  //
  Hero,
  Avatar,
  Pet,
  Race,
  Class,
  //
  Weapon,
  Armor,
  Jewelry,
  Shield,
  //
  Access,
  Present,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum TokenSubType {
  // Jewelry
  Ring,
  Earring,
  Necklace,

  // Armor
  Helmet,
  HelmetLight,
  HelmetHeavy,
  Body,
  BodyLight,
  BodyHeavy,
  Pants,
  PantsLight,
  PantsHeavy,
  Boots,
  BootsLight,
  BootsHeavy,
  Gloves,
  GlovesLight,
  GlovesHeavy,
  Cloak,
  Wristband,
  WristbandLight,
  WristbandHeavy,
  Belt,
  BeltLight,
  BeltHeavy,

  // Weapon
  Wand,
  Castet,
  Knife,
  Sword,
  Sword2,
  Hatchet,
  Hatchet2,
  Cudgel,
  Cudgel2,
  Staff,
  // Gloves

  Shield,

  Pet,
  Race,
  Class,

  // Class
  MagCrit,
  MagDodge,
  Tank,
  Warrior,
  MonkBuff,
  MonkParry,

  // Pet
  // MagCrit,
  // MagDodge,
  // Tank,
  // Warrior,
  // MonkBuff,
  // MonkParry,

  // Access
  Tester,
  Ladder,
  PreAlphaTester,
  AlphaTester,
  BetaTester,

  // Present
  Cup,
  Pen,
  Camera,

  // Race
  Human,
  Elf,
  Dwarf,
  Giant,
  BeastMan,
  Werewolf,
}

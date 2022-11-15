use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{unordered_map::UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, Timestamp, serde::{Deserialize, Serialize}};

mod community;
mod for_sale;
mod housing;
mod jobs;

pub use crate::community::*;
pub use crate::for_sale::*;
pub use crate::housing::*;
pub use crate::jobs::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    community: UnorderedMap<String, Vector<Community>>,
    housing: UnorderedMap<String, Vector<Housing>>,
    jobs: UnorderedMap<String, Vector<Job>>,
    for_sale: UnorderedMap<String, Vector<ForSale>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct ItemInfo {
    creator: AccountId,
    post_id: u64,
    date: Timestamp,
    category: String,
    title: String,
    description: String,
    image: Option<Vec<String>>,
    location: Option<String>,
    price: Option<u64>,
    postal_code: Option<u16>,
    city_or_neighborhood: Option<String>,
}

impl Default for Contract {
    // The default trait with which to initialize the contract
    fn default() -> Self {
        Self {
            community: UnorderedMap::new(b"community".to_vec()),
            housing: UnorderedMap::new(b"housing".to_vec()),
            jobs: UnorderedMap::new(b"jobs".to_vec()),
            for_sale: UnorderedMap::new(b"for_sale".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {}


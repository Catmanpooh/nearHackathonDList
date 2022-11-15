use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, require};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Community {
    item_info: ItemInfo,
    garage_sale: Option<GarageOrMovingSales>,
    class_or_event: Option<ClassesOrEvents>,
    lost_or_found: Option<bool>,
    rideshare: Option<bool>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GarageOrMovingSales {
    garage_sale_start_time: Option<String>,
    garage_sale_dates: Option<Vec<String>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ClassesOrEvents {
    event_venue: Option<String>,
    event_start_date: Option<String>,
    event_duration: Option<u8>,
    event_features: Option<Vec<String>>,
}

trait CommunityInfo {
    fn get_community_item(&self, category: String, post_id: u64) -> Option<Community>;

    fn get_all_community_items(&self, category: String) -> Vec<Community>;

    fn set_community_item(&mut self, community_info_in: Community);

    fn remove_community_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String>;
}

#[near_bindgen]
impl CommunityInfo for Contract {
    fn get_community_item(&self, category: String, post_id: u64) -> Option<Community> {
        match self.community.get(&category) {
            Some(x) => x.iter().find(|y| y.item_info.post_id == post_id),
            None => None,
        }
    }

    fn get_all_community_items(&self, category: String) -> Vec<Community> {
        match self.community.get(&category) {
            Some(x) => x.to_vec(), 
            None => vec![]
        }
    }

    #[payable]
    fn set_community_item(&mut self, community_info_in: Community) {
        let account_id = env::signer_account_id();

        
        // Get all items associated with the group's category if none return empty vector.
        let mut temp_list = match self.community.get(&community_info_in.item_info.category) {
            Some(x) => x,
            None => Vector::new(env::sha256(account_id.as_bytes())),
        };

        // Pushing information into vector.
        temp_list.push(&community_info_in);

        self.community.insert(&community_info_in.item_info.category, &temp_list);
        
    }

    fn remove_community_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String> {
        require!(
            env::signer_account_id().to_string() == account_id,
            "can not delete item check that account id is equal to yours!"
        );

        match self.community.get(&category) {
            Some(mut x) => {
                if let Some(index) = x.iter().position(|y| y.item_info.post_id == post_id) {
                    x.swap_remove(index.try_into().unwrap());
                    self.community.insert(&category, &x);

                    Some("Items has been successfully removed".to_string())

                } else {
                    None
                }
            }
            None => None,
        }
    }
}

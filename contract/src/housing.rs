use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, require};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Housing {
    item_info: ItemInfo,
    rent: Option<u64>,
    per_time_range: Option<PerTimeRange>,
    sqft: Option<u64>,
    pet: Option<bool>,
    air_conditioning: Option<bool>,
    private_room: Option<bool>,
    housing_type: Option<HousingType>,
    laundry: Option<bool>,
    parking: Option<bool>,
    available_date: Option<String>,
    open_house_dates: Option<Vec<String>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum PerTimeRange {
    Day,
    Week,
    Month,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum HousingType {
    Apartment,
    Condo,
    CottageOrCabin,
    Duplex,
    Flat,
    House,
    InLaw,
    Loft,
    TownHouse,
    Manufactured,
    AssistedLiving,
    Land,
}

trait HousingInfo {
    fn get_housing_item(&self, category: String,  post_id: u64) -> Option<Housing>;

    fn get_all_housing_items(&self, category: String, ) -> Vec<Housing>;

    fn set_housing_item(&mut self, housing_info_in: Housing);

    fn remove_housing_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String>;
}

#[near_bindgen]
impl HousingInfo for Contract {
    fn get_housing_item(&self, category: String,  post_id: u64) -> Option<Housing> {
        match self.housing.get(&category) {
            Some(x) => x.iter().find(|y| y.item_info.post_id == post_id),
            None => None,
        }
    }

    fn get_all_housing_items(&self, category: String) -> Vec<Housing> {
        match self.housing.get(&category) {
            Some(x) => x.to_vec(), 
            None => vec![]
        }
    }

    #[payable]
    fn set_housing_item(&mut self, housing_info_in: Housing) {
        let account_id = env::signer_account_id();

        // Get all items associated with the group's category if none return empty vector.
        let mut temp_list = match self.housing.get(&housing_info_in.item_info.category) {
            Some(x) => x,
            None => Vector::new(env::sha256(account_id.as_bytes())),
        };

        // Pushing information into vector.
        temp_list.push(&housing_info_in);

        self.housing.insert(&housing_info_in.item_info.category, &temp_list);
      
    }

    fn remove_housing_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String> {
        require!(
            env::signer_account_id().to_string() == account_id,
            "can not delete item check that account id is equal to yours!"
        );

        match self.housing.get(&category) {
            Some(mut x) => {
                if let Some(index) = x.iter().position(|y| y.item_info.post_id == post_id) {
                    
                    x.swap_remove(index.try_into().unwrap());
                    self.housing.insert(&category, &x);

                    Some("Items has been successfully removed".to_string())

                } else {
                    None
                }
            }
            None => None,
        }
    }
}

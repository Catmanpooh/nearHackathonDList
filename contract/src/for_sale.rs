use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, require};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ForSale {
    item_info: ItemInfo,
    make_or_manufacturer: Option<String>,
    model_name_or_number: Option<String>,
    size_dimensions: Option<String>,
    condition: Option<Condition>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Condition {
    New,
    LikeNew,
    Excellent,
    Good,
    Fair,
    Salvage,
}

trait ForSaleInfo {
    fn get_for_sale_item(&self, category: String, post_id: u64) -> Option<ForSale>;

    fn get_all_for_sale_items(&self, category: String) -> Vec<ForSale>;

    fn set_for_sale_item(&mut self, for_sale_info_in: ForSale);

    fn remove_for_sale_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String>;
}

#[near_bindgen]
impl ForSaleInfo for Contract {
    fn get_for_sale_item(&self, category: String, post_id: u64) -> Option<ForSale> {
        match self.for_sale.get(&category) {
            Some(x) => x.iter().find(|y| y.item_info.post_id == post_id),
            None => None,
        }
    }

    fn get_all_for_sale_items(&self, category: String) -> Vec<ForSale> {
        match self.for_sale.get(&category) {
            Some(x) => x.to_vec(), 
            None => vec![]
        }
    }

    #[payable]
    fn set_for_sale_item(&mut self, for_sale_info_in: ForSale) {
        let account_id = env::signer_account_id();

        // Get all items associated with the group's category if none return empty vector.
        let mut temp_list = match self.for_sale.get(&for_sale_info_in.item_info.category) {
            Some(x) => x,
            None => Vector::new(env::sha256(account_id.as_bytes())),
        };

        // Pushing information into vector.
        temp_list.push(&for_sale_info_in);

        self.for_sale.insert(&for_sale_info_in.item_info.category, &temp_list);
      
    }

    fn remove_for_sale_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String> {
        require!(
            env::signer_account_id().to_string() == account_id,
            "can not delete item check that account id is equal to yours!"
        );

        match self.for_sale.get(&category) {
            Some(mut x) => {
                if let Some(index) = x.iter().position(|y| y.item_info.post_id == post_id) {
                    x.swap_remove(index.try_into().unwrap());
                    self.for_sale.insert(&category, &x);

                    Some("Items has been successfully removed".to_string())

                } else {
                    None
                }
            }
            None => None,
        }
    }
}

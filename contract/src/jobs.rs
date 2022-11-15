use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, require};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Job {
    pub item_info: ItemInfo,
    pub employment_type: Option<EmploymentType>,
    pub job_title: Option<String>,
    pub compensation: Option<u64>,
    pub company_name: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum EmploymentType {
    FullTime,
    PartTime,
    ContractWork,
    EmployeeChoice,
}

trait JobInfo {
    fn get_jobs_item(&self, category: String,  post_id: u64) -> Option<Job>;

    fn get_all_jobs_items(&self, category: String) -> Vec<Job>;

    fn set_jobs_item(&mut self, jobs_info_in: Job);

    fn remove_jobs_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String>;
}

#[near_bindgen]
impl JobInfo for Contract {
    fn get_jobs_item(&self, category: String, post_id: u64) -> Option<Job> {
        match self.jobs.get(&category) {
            Some(x) => x.iter().find(|y| y.item_info.post_id == post_id),
            None => None,
        }
    }

    fn get_all_jobs_items(&self, category: String) -> Vec<Job> {

        match self.jobs.get(&category) {
                Some(x) => x.to_vec(), 
                None => vec![]
            }

    }

    #[payable]
    fn set_jobs_item(&mut self, jobs_info_in: Job) {
        let account_id = env::signer_account_id();
        
        // Get all items associated with the group's category if none return empty vector.
        let mut temp_list = match self.jobs.get(&jobs_info_in.item_info.category) {
            Some(x) => x,
            None => Vector::new(env::sha256(account_id.as_bytes())),
        };

        // Pushing information into vector.
        temp_list.push(&jobs_info_in);

        self.jobs.insert(&jobs_info_in.item_info.category, &temp_list);

    }

    fn remove_jobs_item(&mut self, category: String, account_id: String, post_id: u64) -> Option<String> {
        require!(
            env::signer_account_id().to_string() == account_id,
            "can not delete item check that account id is equal to yours!"
        );


        match self.jobs.get(&category) {
            Some( mut x) => { 
                if let Some(index) = x.iter().position(|y| y.item_info.post_id == post_id) {
                    
                    /*  
                        Issue with the vector when trying to fetch from get all items.
                        UnorderedMap is not updating so the len is out of bound.
                        Quick solution clear vector and reinsert || just insert back into category.
                    */
                    x.swap_remove(index.try_into().unwrap());
                    self.jobs.insert(&category, &x);

                    Some("Items has been successfully removed".to_string())
                } else {
                    None
                }
            }
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn set_then_get_lists() {
        let mut contract = Contract::default();
        let account_id = env::current_account_id();
        let job_pass_in = Job {
            item_info: ItemInfo { 
                creator: account_id, 
                post_id: 365416516352, 
                date: env::block_timestamp(), 
                category: "finance".to_string(),
                title: "Looking for a financal tech".to_string(), 
                description: "This job is looking for a person to do all the money making I sit back and watch".to_string(),
                image: None,
                location: Some("remote".to_string()),
                price: None,
                postal_code: Some(45621),
                city_or_neighborhood: None 
            },
            employment_type: Some(EmploymentType::FullTime),
            job_title: Some("Make Me Money".to_string()),
            compensation: Some(3500),
            company_name: Some("Make Money Wedenday".to_string()),
        };

        let job_pass_in2 = Job {
            item_info: ItemInfo { 
                creator: env::current_account_id(), 
                post_id: 952616546515, 
                date: env::block_timestamp(), 
                category: "finance".to_string(),
                title: "Looking for a financal tech".to_string(), 
                description: "This job is looking for a person to do all the money making I sit back and watch".to_string(),
                image: None,
                location: Some("remote".to_string()),
                price: None,
                postal_code: Some(45621),
                city_or_neighborhood: None 
            },
            employment_type: Some(EmploymentType::FullTime),
            job_title: Some("Make Me Money".to_string()),
            compensation: Some(4200),
            company_name: Some("Make Money Wedenday".to_string()),
        };

        let job_pass_in3 = Job {
            item_info: ItemInfo { 
                creator: env::current_account_id(), 
                post_id: 652146546552, 
                date: env::block_timestamp(), 
                category: "finance".to_string(),
                title: "Looking for a financal tech".to_string(), 
                description: "This job is looking for a person to do all the money making I sit back and watch".to_string(),
                image: None,
                location: Some("remote".to_string()),
                price: None,
                postal_code: Some(45621),
                city_or_neighborhood: None 
            },
            employment_type: Some(EmploymentType::FullTime),
            job_title: Some("Make Me Money".to_string()),
            compensation: Some(4200),
            company_name: Some("Make Money Wedenday".to_string()),
        };
        contract.set_jobs_item( job_pass_in);
        contract.set_jobs_item( job_pass_in2);
        contract.set_jobs_item( job_pass_in3);

        let j = contract.get_all_jobs_items("finance".to_string());
        let r = contract.remove_jobs_item("finance".to_string(), env::signer_account_id().to_string(), 952616546515);
        assert_eq!(j.len(),  3);
        assert_eq!(r, Some("Items has been successfully removed".to_string()));
        let l = contract.get_jobs_item("finance".to_string(), 952616546515);
        assert_eq!(l, None);
        let j2 = contract.get_all_jobs_items("finance".to_string());
        assert_eq!(j2.len(),  2);
    

    }
}
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: LookupMap<AccountId, String>,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r"),
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: &AccountId) -> Option<String> {
        return self.records.get(&account_id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    #[test]
    fn set_get_message() {
        let bob: AccountId = "bob".parse().unwrap();
        let context = VMContextBuilder::new()
            .signer_account_id(bob.clone())
            .build();
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!("hello".to_string(), contract.get_status(&bob).unwrap());
    }

    #[test]
    fn get_nonexistent_message() {
        let context = VMContextBuilder::new().is_view(true).build();
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status(&"francis.near".parse().unwrap()));
    }
}

mod event;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{AccountId, env, near_bindgen, BorshStorageKey};

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Records
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: LookupMap<AccountId, String>,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            records: LookupMap::new(StorageKey::Records),
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, message: String) {
        let account_id = env::predecessor_account_id();
        self.records.insert(&account_id, &message);
        event::emit::set_account_status(&account_id, message);
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.records.get(&account_id)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(1))
            .predecessor_account_id(accounts(2));
        builder
    }

    #[test]
    fn set_get_message() {
        let context = get_context();
        testing_env!(context.build());
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status(accounts(2)).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context();
        testing_env!(context.build());
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status(accounts(4)));
    }
}

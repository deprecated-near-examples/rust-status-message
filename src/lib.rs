use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldStatusMessage {
    records: LookupMap<String, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    taglines: LookupMap<String, String>,
    bios: LookupMap<String, String>,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            taglines: LookupMap::new(b"r".to_vec()),
            bios: LookupMap::new(b"b".to_vec()),
        }
    }
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_tagline(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.taglines.insert(&account_id, &message);
    }

    pub fn get_tagline(&self, account_id: String) -> Option<String> {
        return self.taglines.get(&account_id);
    }

    pub fn set_bio(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.bios.insert(&account_id, &message);
    }

    pub fn get_bio(&self, account_id: String) -> Option<String> {
        return self.bios.get(&account_id);
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldStatusMessage = env::state_read().expect("failed");
        Self {
            taglines: old_state.records,
            bios: LookupMap::new(b"b".to_vec()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_tagline() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_tagline("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_tagline("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_tagline() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_tagline("francis.near".to_string()));
    }

    #[test]
    fn set_get_bio() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_bio("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_bio("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_bio() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_bio("francis.near".to_string()));
    }
}

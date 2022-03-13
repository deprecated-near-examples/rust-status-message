pub mod emit {
    use near_sdk::log;
    use near_sdk::serde_json::json;

    use crate::*;

    pub fn set_account_status(account_id: &AccountId, message: String) {
        let event = json!({
            "standard": "",
            "version": "1.0.0",
            "event": "set_status",
            "data": [
                {"account_id": account_id, "message": message}
            ]
        });

        log!("EVENT_JSON:{}", event.to_string());
    }

}

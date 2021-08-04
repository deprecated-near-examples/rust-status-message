Status Message in Rust - Gitpod version
=======================================

This smart contract saves and records the status messages of NEAR accounts that call it.

**Note**: this README is specific to Gitpod and this example. For local development, please see [README.md](README.md).

## Using this contract

### Web app

Deploy the smart contract to a specific account created with the NEAR Wallet. Then interact with the smart contract using near-api-js on the frontend.

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Deploy the contract to your NEAR account:

```bash
near deploy --wasmFile res/status_message.wasm --accountId YOUR_ACCOUNT_NAME
```

Build the frontend:

```bash
npm start
```

If all is successful the app should be live at `localhost:1234`!

### CLI

In Gitpod, a process has automatically created a new NEAR account that's useful for a quick (and likely temporary) usage.
We've set an environment variable in Gitpod with the account name. At the bottom of this screen there's a Terminal.

You may see the NEAR account by running this command:
```bash
echo $CONTRACT_NAME
```

The next command will call the contract's `set_status` method:

```bash
near call $CONTRACT_NAME set_status '{"message": "aloha!"}' --accountId $CONTRACT_NAME
```

To retrieve the message from the contract, call `get_status` with the following:

```bash
near view $CONTRACT_NAME get_status '{"account_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME
```

Note that these status messages are stored per account in a `HashMap`. See `src/lib.rs` for the code. We can try the same steps with another account to verify.
**Note**: we're adding `NEW_ACCOUNT_NAME` for the next couple steps.

There are two ways to create a new account:
 - the NEAR Wallet (as we did before)
 - `near create_account NEW_ACCOUNT_NAME --masterAccount $CONTRACT_NAME`

Now call the contract on the first account (where it's deployed):

```bash
near call $CONTRACT_NAME set_status '{"message": "bonjour"}' --accountId NEW_ACCOUNT_NAME
```

```bash
near view $CONTRACT_NAME get_status '{"account_id": "NEW_ACCOUNT_NAME"}'
```

Returns `bonjour`.

Make sure the original status remains:

```bash
near view $CONTRACT_NAME get_status '{"account_id": "$CONTRACT_NAME"}'
```

Now that you've seen this working in Gitpod, feel free to clone this repository and use it as a starting point for your own project.

## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```

## Data collection
By using Gitpod in this project, you agree to opt-in to basic, anonymous analytics. No personal information is transmitted. Instead, these usage statistics aid in discovering potential bugs and user flow information.

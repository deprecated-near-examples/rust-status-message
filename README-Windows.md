Status Message
==============

This smart contract saves and records the status messages of NEAR accounts that call it.

**Note**: this README is specific to Windows and this example. For development on OS X or Linux, please see [README.md](README.md).

## Prerequisites
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Install dependencies

```
npm install
```

## Building this contract
To make the build process compatible with multiple operating systems, the build process exists as a script in `package.json`.
There are a number of special flags used to compile the smart contract into the wasm file.
Run this command to build and place the wasm file in the `res` directory:
```bash
npm run build
```

**Note**: Instead of `npm`, users of [yarn](https://yarnpkg.com) may run:
```bash
yarn build
```

### Important
If you encounter an error similar to:
>note: the `wasm32-unknown-unknown` target may not be installed

Then run:

```bash
rustup target add wasm32-unknown-unknown
```

## Using this contract

### Web app

Deploy the smart contract to a specific account created with the NEAR Wallet. Then interact with the smart contract using near-api-js on the frontend.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

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

### Quickest deploy
Build and deploy this smart contract to an development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/status_message.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account.env`. To make the next few steps easier, we're going to set an environment variable containing this development account id and use that when copy/pasting commands.

If the account name is not immediately visible on the Command Prompt, you may find it by running:

```bash
type neardev\dev-account.env
```

It will display something similar to `CONTRACT_NAME=dev-12345678901234`.
Please set the Windows environment variable by copying that value and running `set` like so:

```bash
set CONTRACT_NAME=dev-12345678901234
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo %CONTRACT_NAME%
```

The next command will call the contract's `set_status` method:

```bash
near call %CONTRACT_NAME% set_status "{\"message\": \"aloha!\"}" --accountId %CONTRACT_NAME%
```

**Note**: at the time of this writing, Windows does not handle single quotes `'` well, so these commands must use escaped double-quotes `\"` which, as you may know, equates to a regular double quote `"` when parsed. Apologies for some of the unsightly commands, but it's out of necessity.

To retrieve the message from the contract, call `get_status` with the following:

```bash
near view %CONTRACT_NAME% get_status "{\"account_id\": \""%CONTRACT_NAME%"\"}" --accountId %CONTRACT_NAME%
```

### Standard deploy
In this option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

```
near login
```

Deploy the contract:

```bash
near deploy --wasmFile res/status_message.wasm --accountId YOUR_ACCOUNT_NAME
```

Set a status for your account:

```bash
near call YOUR_ACCOUNT_NAME set_status "{\"message\": \"aloha friend\"}" --accountId YOUR_ACCOUNT_NAME
```

Get the status:

```bash
near view YOUR_ACCOUNT_NAME get_status "{\"account_id\": \"YOUR_ACCOUNT_NAME\"}"
```

Note that these status messages are stored per account in a `HashMap`. See `src/lib.rs` for the code. We can try the same steps with another account to verify.
**Note**: we're adding `NEW_ACCOUNT_NAME` for the next couple steps.

There are two ways to create a new account:
 - the NEAR Wallet (as we did before)
 - `near create_account NEW_ACCOUNT_NAME --masterAccount YOUR_ACCOUNT_NAME`

Now call the contract on the first account (where it's deployed):

```bash
near call YOUR_ACCOUNT_NAME set_status "{\"message\": \"bonjour\"}" --accountId NEW_ACCOUNT_NAME
```

```bash
near view YOUR_ACCOUNT_NAME get_status "{\"account_id\": \"NEW_ACCOUNT_NAME\"}"
```

Returns `bonjour`.

Make sure the original status remains:

```bash
near view YOUR_ACCOUNT_NAME get_status "{\"account_id\": \"YOUR_ACCOUNT_NAME\"}"
```

## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```

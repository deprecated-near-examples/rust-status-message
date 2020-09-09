Status Message
==============

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/near-examples/rust-status-message)

<!-- MAGIC COMMENT: DO NOT DELETE! Everything above this line is hidden on NEAR Examples page -->

This smart contract saves and records the status messages of NEAR accounts that call it.

Windows users: please visit the [Windows-specific README file](README-Windows.md).

## Prerequisite
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
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

## Using this contract

### Quickest deploy
Build and deploy this smart contract to an development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/status_message.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an environment variable containing this development account id and use that when copy/pasting commands.
Run this command to the environment variable:

```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME
```

The next command will call the contract's `set_status` method:

```bash
near call $CONTRACT_NAME set_status '{"message": "aloha!"}' --accountId $CONTRACT_NAME
```

To retrieve the message from the contract, call `get_status` with the following:

```bash
near view $CONTRACT_NAME get_status '{"account_id": "'$CONTRACT_NAME'"}'
```

### Standard deploy
In this second option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.nearprotocol.com).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Deploy the contract:

```bash
near deploy --wasmFile res/status_message.wasm --accountId YOUR_ACCOUNT_NAME
```

Set a status for your account:

```bash
near call YOUR_ACCOUNT_NAME set_status '{"message": "aloha friend"}' --accountId YOUR_ACCOUNT_NAME
```

Get the status:

```bash
near view YOUR_ACCOUNT_NAME get_status '{"account_id": "YOUR_ACCOUNT_NAME"}'
```

Note that these status messages are stored per account in a `HashMap`. See `src/lib.rs` for the code. We can try the same steps with another account to verify.
**Note**: we're adding `NEW_ACCOUNT_NAME` for the next couple steps.

There are two ways to create a new account:
 - the NEAR Wallet (as we did before)
 - `near create_account NEW_ACCOUNT_NAME --masterAccount YOUR_ACCOUNT_NAME`

Now call the contract on the first account (where it's deployed):

```bash
near call YOUR_ACCOUNT_NAME set_status '{"message": "bonjour"}' --accountId NEW_ACCOUNT_NAME
```

```bash
near view YOUR_ACCOUNT_NAME get_status '{"account_id": "NEW_ACCOUNT_NAME"}'
```

Returns `bonjour`.

Make sure the original status remains:

```bash
near view YOUR_ACCOUNT_NAME get_status '{"account_id": "YOUR_ACCOUNT_NAME"}'
```

## Testing
To test run:
```bash
cargo test --package status-message -- --nocapture
```

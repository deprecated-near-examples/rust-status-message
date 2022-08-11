#!/bin/bash
echo ==== Quick Deploy ====
TEXT=$(printf 'y\n' | near dev-deploy --wasmFile res/status_message.wasm --helperUrl https://near-contract-helper.onrender.com)
if [[ ! "$TEXT" =~ .*"Done deploying to".* ]]; then
    echo -e "\033[0;31m FAIL \033[0m"
    exit 1
else
    echo -e "\033[0;32m SUCCESS \033[0m"
fi

echo ==== Set dev account env variable ====
source neardev/dev-account.env
TEXT=$(echo $CONTRACT_NAME)
if [[ ! "$TEXT" =~ .*"dev-".* ]]; then
    echo -e "\033[0;31m FAIL \033[0m"
    exit 1
else
    echo -e "\033[0;32m SUCCESS \033[0m"
fi

echo ==== Set status ====
TEXT=$(near call $CONTRACT_NAME set_status '{"message": "aloha!"}' --accountId $CONTRACT_NAME)
if [[ ! "$TEXT" =~ .*"To see the transaction in the transaction explorer".* ]]; then
    echo -e "\033[0;31m FAIL \033[0m"
    exit 1
else
    echo -e "\033[0;32m SUCCESS \033[0m"
fi

echo ==== Get status ====
TEXT=$(near view $CONTRACT_NAME get_status '{"account_id": "'$CONTRACT_NAME'"}')
if [[ ! "$TEXT" =~ .*"aloha!".* ]]; then
    echo -e "\033[0;31m FAIL \033[0m"
    exit 1
else
    echo -e "\033[0;32m SUCCESS \033[0m"
fi

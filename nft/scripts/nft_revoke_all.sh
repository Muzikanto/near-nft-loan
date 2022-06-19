#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="1"
ACCOUNT_ID="muzikant.testnet"
near call $CONTRACT_NAME nft_revoke_all --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\" }" --amount "0.1" --gas 300000000000000

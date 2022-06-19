#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="11"
near call $CONTRACT_NAME nft_mint --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\", \"token_metadata\": { \"title\": \"Token$TOKEN_ID\", \"media\": \"https://images.mfight.io/nft/1082.png\"}, \"token_owner_id\": \"$ACCOUNT_ID\" }" --amount "0.1" --gas 300000000000000

#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="4"
near call $CONTRACT_NAME loan_nft_pay --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT\" }" --amount "0.872" --gas 300000000000000

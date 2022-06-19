#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="6"
near call $CONTRACT_NAME loan_nft --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT\" }" --gas 300000000000000

#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="4"
near call $CONTRACT_NAME nft_approve --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"account_id\": \"$LOAN_CONTRACT\", \"msg\": \"1\" }" --amount "0.1" --gas 300000000000000

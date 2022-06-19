#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
TOKEN_ID="9269"
near call $CONTRACT_NAME loan_nft_claim_expired --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT2\" }" --gas 300000000000000

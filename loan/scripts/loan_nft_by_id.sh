#!/bin/bash
source neardev/dev-account.env
NFT_CONTRACT="tiger7.testnet"
TOKEN_ID="1955"
near view $CONTRACT_NAME loan_nft_by_id "{ \"contract_id\": \"$NFT_CONTRACT\", \"token_id\": \"$TOKEN_ID\" }"

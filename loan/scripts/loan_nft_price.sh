#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near view $CONTRACT_NAME loan_nft_price "{ \"contract_id\": \"$NFT_CONTRACT\" }"

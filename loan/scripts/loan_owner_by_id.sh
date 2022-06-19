#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="4"
near view $CONTRACT_NAME loan_owner_by_id "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT\" }"

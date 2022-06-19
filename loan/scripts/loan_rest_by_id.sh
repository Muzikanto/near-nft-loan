#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="7"
near view $CONTRACT_NAME loan_rest_by_id "{ \"token_id\": \"$TOKEN_ID\", \"contract_id\": \"$NFT_CONTRACT\" }"

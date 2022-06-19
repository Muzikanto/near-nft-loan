#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME loan_nft_is_whitelist "{ \"contract_id\": \"$NFT_CONTRACT\" }"

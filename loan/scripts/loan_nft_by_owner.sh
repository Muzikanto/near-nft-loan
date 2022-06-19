#!/bin/bash
source neardev/dev-account.env
OWNER_ID="muzikant.testnet"
LIMIT="10"
FROM_INDEX="0"
near view $CONTRACT_NAME loan_nft_by_owner "{ \"account_id\": \"$OWNER_ID\", \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"

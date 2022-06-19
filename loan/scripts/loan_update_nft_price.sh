#!/bin/bash
source neardev/dev-account.env
NFT_CONTRACT="dev-1648577859565-13862973208014"
PRICE="1000000000000000000000000"
PERCENT="20"
near call $CONTRACT_NAME loan_update_nft_price --accountId $CONTRACT_NAME "{ \"contract_id\": \"$NFT_CONTRACT\", \"price\": \"$PRICE\", \"percent\": $PERCENT }" --gas 300000000000000

#!/bin/bash
source neardev/dev-account.env
NFT_CONTRACT="dev-1648577859565-13862973208014"
near call $CONTRACT_NAME loan_nft_whitelist_add --accountId $CONTRACT_NAME "{ \"contract_id\": \"$NFT_CONTRACT\"}" --depositYocto=1

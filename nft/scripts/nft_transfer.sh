#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="nft_tests.testnet"
RECEIVER_ID="muzikant.testnet"
TOKEN_ID="1689"

near call $CONTRACT_NAME nft_transfer --accountId $ACCOUNT_ID "{ \"token_id\": \"$TOKEN_ID\", \"receiver_id\":\"$RECEIVER_ID\" }" --depositYocto 1

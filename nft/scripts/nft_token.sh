#!/bin/bash
source neardev/dev-account.env
#CONTRACT_NAME="tiger7.testnet"
TOKEN_ID="11"
near view $CONTRACT_NAME nft_token "{ \"token_id\": \"$TOKEN_ID\" }"

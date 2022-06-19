#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
AMOUNT="1000000000000000000000000"
near call $CONTRACT_NAME loan_withdraw --accountId $ACCOUNT_ID "{ \"amount\": \"$AMOUNT\" }"

#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near call $CONTRACT_NAME loan_withdraw_all --accountId $ACCOUNT_ID "{ }"

#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near call $CONTRACT_NAME loan_deposit --accountId $ACCOUNT_ID "{}" --amount "1"

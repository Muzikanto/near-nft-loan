#!/bin/bash
source neardev/dev-account.env
ACCOUNT_ID="muzikant.testnet"
near call $CONTRACT_NAME loan_claim_rewards --accountId $ACCOUNT_ID "{ }" --gas 300000000000000

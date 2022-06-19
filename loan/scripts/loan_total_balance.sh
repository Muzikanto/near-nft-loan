#!/bin/bash
source neardev/dev-account.env

near view $CONTRACT_NAME loan_total_balance "{  }"

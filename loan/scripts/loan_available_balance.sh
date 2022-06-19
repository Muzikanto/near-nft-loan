#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME loan_available_balance "{  }"

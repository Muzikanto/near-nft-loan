#!/bin/bash
source neardev/dev-account.env
CONTRACT_NAME="tiger7.testnet"
near view $CONTRACT_NAME nft_metadata "{ }"

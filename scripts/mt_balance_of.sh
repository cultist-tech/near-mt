#!/bin/bash
source neardev/dev-account.env

TOKEN_ID1="mfight-ft.testnet"
TOKEN_ID2="mfight-xp.testnet"
ACCOUNT_ID="muzikant.testnet"

near view $CONTRACT_NAME mt_balance_of --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID1\", \"account_id\": \"$ACCOUNT_ID\" }"
near view $CONTRACT_NAME mt_balance_of --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID2\", \"account_id\": \"$ACCOUNT_ID\" }"


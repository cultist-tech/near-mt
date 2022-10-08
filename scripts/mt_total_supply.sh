#!/bin/bash
source neardev/dev-account.env

TOKEN_ID1="mfight-ft.testnet"
TOKEN_ID2="mfight-xp.testnet"

near view $CONTRACT_NAME mt_total_supply --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID1\" }"
near view $CONTRACT_NAME mt_total_supply --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID2\" }"


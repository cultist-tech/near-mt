#!/bin/bash
source neardev/dev-account.env

#TOKEN_ID_1="mfight-ft.testnet"
#near call $CONTRACT_NAME mt_add_token --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID\" }"

TOKEN_ID_2="mfight-xp.testnet"
near call $CONTRACT_NAME mt_add_token --accountId $CONTRACT_NAME "{ \"token_id\": \"$TOKEN_ID_2\" }"


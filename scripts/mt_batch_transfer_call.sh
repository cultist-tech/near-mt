#!/bin/bash
source neardev/dev-account.env

NFT_CONTRACT="mfight-nft_v2.testnet"
TOKEN_ID_1="mfight-ft.testnet"
TOKEN_ID_2="mfight-xp.testnet"
AMOUNT1="2000000000000000000000000"
AMOUNT2="2000000000000000000000000"
ACCOUNT_ID="muzikant.testnet"

near call $CONTRACT_NAME mt_batch_transfer_call --accountId $ACCOUNT_ID "{ \"receiver_id\": \"$NFT_CONTRACT\", \"token_ids\": [\"$TOKEN_ID_1\", \"$TOKEN_ID_2\"], \"amounts\": [\"$AMOUNT1\", \"$AMOUNT2\"], \"msg\": \"{\\\"token_id\\\":\\\"1017\\\"}\" }" --depositYocto "1" --gas 300000000000000


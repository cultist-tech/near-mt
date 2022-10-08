#!/bin/bash
source neardev/dev-account.env

near call $CONTRACT_NAME fix --accountId $CONTRACT_NAME "{ }"


#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="$CONTRACT_NAME"
PAUSE="true"

near call $CONTRACT_NAME set_is_paused --accountId $ "{ \"pause\": $PAUSE }"
near call $CONTRACT_NAME is_paused --accountId $ACCOUNT_ID "{ }"


#!/bin/bash

source "$(dirname "$0")/../../.env"

# Check if the script is run with "dev" argument
if [ "$1" == "dev" ]; then
  private_key="$STYLUS_DEV_PK"
  rpc_url="$RPC_DEV_URL"
else
  private_key="$STYLUS_LOCAL_DEV_PK"
  rpc_url="$RPC_URL"
fi

cast send --rpc-url $rpc_url --private-key $private_key $USER_2_ADDRESS --value 1000000000000000000
cast send --rpc-url $rpc_url --private-key $private_key $USER_3_ADDRESS --value 1000000000000000000
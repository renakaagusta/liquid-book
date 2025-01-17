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

echo "Initialize engine"
cast send $ENGINE_ADDRESS "initialize(address,address,address,address)" $TICK_ADDRESS $ORDER_ADDRESS $BITMAP_ADDRESS $MATCHER_ADDRESS --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Initialize matcher"
cast send $MATCHER_ADDRESS "initialize(address,address,address,address)" $TICK_ADDRESS $BITMAP_ADDRESS $ORDER_ADDRESS $POOL_ORDERBOOK_ADDRESS --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Initialize tick"
cast send $TICK_ADDRESS "initialize(address,address,address)" $ENGINE_ADDRESS $BITMAP_ADDRESS $ORDER_ADDRESS --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Initialize order"
cast send $ORDER_ADDRESS "initialize(address,address,address)" $ENGINE_ADDRESS $BITMAP_ADDRESS $TICK_ADDRESS --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Initialize pool manager"
cast send $POOL_MANAGER_ADDRESS "initialize(address)" $BALANCE_MANAGER_ADDRESS --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Add new pool"
cast send $POOL_MANAGER_ADDRESS "addPool(address,address,address,address,address,int128,uint256)" $POOL_ORDERBOOK_ADDRESS $MOCK_WETH_ADDRESS $MOCK_USDC_ADDRESS $ENGINE_ADDRESS $BITMAP_ADDRESS 81200 1000000000000000 --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Set initial tick"
cast send --rpc-url $rpc_url --private-key $private_key $BITMAP_ADDRESS "setCurrentTick(int128)" 81200 > /dev/null 2>&1

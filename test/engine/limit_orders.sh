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

sum=0
count=0

process_gas() {
    local output=$1
    local gas=$(echo "$output" | grep "^cumulativeGasUsed" | awk '{print $2}')
    
    echo "$output"

    if [ ! -z "$gas" ]; then
        if [[ $gas == 0x* ]]; then
            gas=$((gas))
        fi
        sum=$((sum + gas))
        count=$((count + 1))
        echo "Gas used: $gas"
    fi
}

# Set operator
echo "Setting operator of user 2.."
cast send --rpc-url $rpc_url --private-key $USER_2_PK $BALANCE_MANAGER_ADDRESS "setOperator(address,bool)" $ENGINE_ADDRESS true  > /dev/null 2>&1

echo "Setting operator of user 3.."
cast send --rpc-url $rpc_url --private-key $USER_3_PK $BALANCE_MANAGER_ADDRESS "setOperator(address,bool)" $ENGINE_ADDRESS true  > /dev/null 2>&1

echo "Place sell orders..."
for tick in $(seq 81400 -20 81200); do
  output=$(cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key $ENGINE_ADDRESS "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 $USER_ADDRESS false false 2>&1)
  process_gas "$output"
done

echo "Place buy orders..."
for tick in $(seq 81180 -20 81000); do
  output=$(cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key $ENGINE_ADDRESS "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 $USER_ADDRESS true false 2>&1)
  process_gas "$output"
done

echo "Place sell orders..."
for tick in $(seq 81200 -20 81000); do
    output=$(cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key $ENGINE_ADDRESS "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 $USER_ADDRESS false false 2>&1)
    process_gas "$output"
done

echo "Place buy orders..."
for tick in $(seq 80900 20 81100); do
  output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key $ENGINE_ADDRESS "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 $USER_ADDRESS true false 2>&1)
  process_gas "$output"
done
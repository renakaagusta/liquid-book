#!/bin/bash

# set -x

private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
rpc_url="http://localhost:8547"

# private_key="45d37ea082249aa1349f24663fbcfdc325b4bce530527e929c4356fc925f4f47"
# rpc_url="https://arb-sepolia.g.alchemy.com/v2/jBG4sMyhez7V13jNTeQKfVfgNa54nCmF"

sum=0
count=0

process_gas() {
    local output=$1
    local gas=$(echo "$output" | grep "^cumulativeGasUsed" | awk '{print $2}')
    
    # echo "$output"

    if [ ! -z "$gas" ]; then
        if [[ $gas == 0x* ]]; then
            gas=$((gas))
        fi
        sum=$((sum + gas))
        count=$((count + 1))
        echo "Gas used: $gas"
    fi
}

# Define the modules
# modules=("bitmap" "engine")
modules=("order" "matcher" "engine" "bitmap" "tick")

# Define the deployment command
deploy_command="cargo stylus deploy -e \$rpc_url --private-key \$private_key --no-verify"

# Declare an associative array to store addresses
declare -A addresses

# Loop through each module and deploy
for module in "${modules[@]}"; do
  echo "Deploying $module..."
  output=$(cd "$(dirname "$0")/$module" && eval $deploy_command 2>&1)
  
  # Parse the deployed contract address and remove color codes
  address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployed code at address:/ {print $NF}')

  # Store the address in the associative array
  addresses[$module]=$address
  
  echo "Deployed $module at address: ${addresses[$module]}"
done

# Print all addresses
echo "All deployed addresses:"
for module in "${!addresses[@]}"; do
  snake_case_module=$(echo "$module" | sed -r 's/([a-z])([A-Z])/\1_\2/g; s/-/_/g' | tr '[:lower:]' '[:upper:]')
  echo "export "${snake_case_module}_ADDRESS=${addresses[$module]}""
done

echo "Initialize contracts"

# Initialize engine
cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize matcher
cast send "${addresses[matcher]}" "initialize(address,address)" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize tick
cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize order
cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1 

echo "Set initial tick"
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(int128)" 100 > /dev/null 2>&1

echo "Place sell orders..."
# Place sell orders (false = sell)
for tick in {110..100}; do
  output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 2>&1)
  process_gas "$output"
done

echo "Place buy orders..."
# Place buy orders (true = buy)
for tick in {99..90}; do
  output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
  process_gas "$output"
done

echo "Get top best ticks for seller"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" false 

echo "Get top best ticks for buyer"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true 

echo "Get bitmap #1"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0

echo "Get tick #1" 
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 

echo "Place limit buy order to fill sell order at tick 101"
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 101 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false > /dev/null 2>&1

output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 101 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
process_gas "$output"

echo "Get bitmap #2"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1

echo "Get tick #2"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1

echo "Place limit sell order to fill buy order at tick 98 (The order book will use 100 to fill this order since it was the best buy tick)" > /dev/null 2>&1
output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" -- -100 1000000000 "0xF63fb6da9b0EEdD4786C8ee464962b5E1b17AD1d" true false 2>&1)
process_gas "$output"

echo "Get bitmap #3"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1

echo "Get tick #3"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1

echo "Place sell orders..."
# Place buy orders (true = buy)
for tick in {99..89}; do
  echo "placed sell order tick: $tick"
  output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 2>&1)
  process_gas "$output"
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" false > /dev/null 2>&1
done

cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true > /dev/null 2>&1

echo "Place buy orders..."
# Place buy orders (true = buy)
for tick in {91..92}; do
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "log(int32)" $tick > /dev/null 2>&1
  output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
  process_gas "$output"
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true > /dev/null 2>&1
done

echo "Total gas used: $sum"
[ $count -gt 0 ] && echo "Average gas used: $((sum / count))" || echo "No gas usage found"
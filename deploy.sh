#!/bin/bash

# set -x

# private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
private_key="45d37ea082249aa1349f24663fbcfdc325b4bce530527e929c4356fc925f4f47"
rpc_url="https://arb-sepolia.g.alchemy.com/v2/jBG4sMyhez7V13jNTeQKfVfgNa54nCmF"

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
  
  # Print the output for debugging
  # echo "Deployment output for $module:"
  # echo "$output"
  
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
# cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key
cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize matcher
# cast send "${addresses[matcher]}" "initialize(address,address)" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key 
cast send "${addresses[matcher]}" "initialize(address,address)" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize tick
# cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key 
cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize order
# cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key 
cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1 

echo "Set initial tick"
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(int128)" 100 > /dev/null 2>&1
# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(uint256)" 100  > /dev/null 2>&1

# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 100 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 

echo "Place sell orders..."
# Place sell orders (false = sell)
for tick in {105..100}; do
  # cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(uint256,uint256,address,bool,bool)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false
  cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false  > /dev/null 2>&1
done

echo "Place buy orders..."
# Place buy orders (true = buy)
for tick in {99..95}; do
  # cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(uint256,uint256,address,bool,bool)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 
  cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false > /dev/null 2>&1
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
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 101 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false

echo "Get bitmap #2"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 

echo "Get tick #2"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 

echo "Place limit sell order to fill buy order at tick 98 (The order book will use 100 to fill this order since it was the best buy tick)"
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 98 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false > /dev/null 2>&1

echo "Get bitmap #3"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 

echo "Get tick #3"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 
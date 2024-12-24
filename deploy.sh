#!/bin/bash

# set -x

private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

# Define the modules
# modules=("bitmap" "engine")
modules=("bitmap" "tick" "order" "matcher" "engine")

# Define the deployment command
deploy_command="cargo stylus deploy -e http://localhost:8547 --private-key \$private_key --no-verify"

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

# Define the RPC URL
rpc_url="http://localhost:8547"

# cast send "${addresses[engine]}" "initialize(address)" "${addresses[bitmap]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize engine
cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize matcher
cast send "${addresses[matcher]}" "initialize(address,address)" "${addresses[tick]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize tick
cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize order
cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

echo "Place sell orders..."
# Place sell orders (false = sell)
for tick in {105..100}; do
  cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeLimitOrder(uint256,uint256,bool)(uint256)" $tick 100 false > /dev/null 2>&1
done

echo "Place buy orders..."
# Place buy orders (true = buy)
for tick in {99..95}; do
  cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeLimitOrder(uint256,uint256,bool)(uint256)" $tick 100 true > /dev/null 2>&1
done

echo "Set initial tick"
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(uint256)" 100 > /dev/null 2>&1
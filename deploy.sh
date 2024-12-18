#!/bin/bash

# set -x

private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

# Define the modules
modules=("bitmap" "engine" "order-manager" "tick-manager")

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
  echo "$module: ${addresses[$module]}"
done

# Define the RPC URL
rpc_url="http://localhost:8547"

cast send "${addresses[engine]}" "initialize(address,address,address)" "${addresses["tick-manager"]}" "${addresses[bitmap]}" "${addresses["order-manager"]}" --rpc-url $rpc_url --private-key $private_key
cast send "${addresses["tick-manager"]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses["order-manager"]}" --rpc-url $rpc_url --private-key $private_key
cast send "${addresses["order-manager"]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses["tick-manager"]}" --rpc-url $rpc_url --private-key $private_key
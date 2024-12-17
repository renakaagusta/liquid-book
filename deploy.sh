#!/bin/bash

private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

# Define the modules
modules=("bitmap" "engine" "order-manager" "tick-mawnager")

# Define the deployment command
deploy_command="cargo stylus deploy -e http://localhost:8547 --private-key \$private_key"

# Declare an associative array to store addresses
declare -A addresses

# Loop through each module and deploy
for module in "${modules[@]}"; do
  echo "Deploying $module..."
  output=$(cd $module && eval $deploy_command 2>&1)
  
  # Print the output for debugging
  # echo "Deployment output for $module:"
  # echo "$output"
  
  # Parse the deployed contract address
  address=$(echo "$output" | awk '/deployed code at address:/ {print $NF}')
  
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

# Echo the cast commands
echo "Run the following cast commands to initialize the contracts (Engine, Tick Manager, Order Manager):"
echo "cast send --rpc-url \"$rpc_url\" --private-key \"$private_key\" \"${addresses[engine]}\" \"initialize(address,address,address)\" \"${addresses["tick-manager"]}\" \"${addresses[bitmap]}\" \"${addresses["order-manager"]}\""
echo "cast send --rpc-url \"$rpc_url\" --private-key \"$private_key\" \"${addresses["tick-manager"]}\" \"initialize(address,address,address)\" \"${addresses[engine]}\" \"${addresses[bitmap]}\" \"${addresses["order-manager"]}\""
echo "cast send --rpc-url \"$rpc_url\" --private-key \"$private_key\" \"${addresses["order-manager"]}\" \"initialize(address,address,address)\" \"${addresses[engine]}\" \"${addresses[bitmap]}\" \"${addresses["tick-manager"]}\""
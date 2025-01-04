#!/bin/bash

# set -x

private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"

# Define the modules
# modules=("bitmap" "engine")
modules=("mock-usdc" "mock-weth")

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
  if ! grep -q "${snake_case_module}_ADDRESS=" .env.example; then
    echo "${snake_case_module}_ADDRESS=${addresses[$module]}" >> .env.example
  else
    sed -i "s|${snake_case_module}_ADDRESS=.*|${snake_case_module}_ADDRESS=${addresses[$module]}|" .env.example
  fi
done

# Define the RPC URL
rpc_url="http://localhost:8547"

# Mint tokens for mock-usdc
cast send "${addresses[mock-usdc]}" "mint(uint256)" 100000 * 1000000 --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Mint tokens for mock-weth
cast send "${addresses[mock-weth]}" "mint(uint256)" 1000 * 1000000000000000000 --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1


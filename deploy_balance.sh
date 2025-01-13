#!/bin/bash

# set -x

# Load environment variables from .env file
source .env

# Check if the script is run with "dev" argument
if [ "$1" == "dev" ]; then
  private_key="$STYLUS_DEV_PK"
  rpc_url="$RPC_DEV_URL"
else
  private_key="$STYLUS_LOCAL_DEV_PK"
  rpc_url="$RPC_URL"
fi

# Define the modules
# modules=("balance-manager" "pool-manager" "pool-orderbook")
modules=("balance-manager")

# Define the deployment command
if [ "$1" == "dev" ]; then
  deploy_command="cargo stylus deploy -e $rpc_url --private-key \$private_key"
else
  deploy_command="cargo stylus deploy -e $rpc_url --private-key \$private_key --no-verify"
fi

# Declare an associative array to store addresses
declare -A addresses

# Loop through each module and deploy
for module in "${modules[@]}"; do
  echo "Deploying $module..."
  output=$(cd "$(dirname "$0")/$module" && eval $deploy_command 2>&1)
  
  # Print the output for debugging
  echo "Deployment output for $module:"
  echo "$output"
  
  # Parse the deployed contract address and remove color codes
  address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployed code at address:/ {print $NF}')

  # Store the address in the associative array
  addresses[$module]=$address
  
  echo "Deployed $module at address: ${addresses[$module]}"
done

# Print all addresses
# Print all addresses and export them to .env
echo "All deployed addresses:"
for module in "${!addresses[@]}"; do
  snake_case_module=$(echo "$module" | sed -r 's/([a-z])([A-Z])/\1_\2/g; s/-/_/g' | tr '[:lower:]' '[:upper:]')
  if ! grep -q "${snake_case_module}_ADDRESS=" .env; then
    echo "export \"${snake_case_module}_ADDRESS=${addresses[$module]}\""
    echo "${snake_case_module}_ADDRESS=${addresses[$module]}" >> .env
  else
    sed -i "s/${snake_case_module}_ADDRESS=.*/${snake_case_module}_ADDRESS=${addresses[$module]}/" .env
  fi
done

# echo "Initialize contracts"
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

# Function to get block number for a transaction
get_block_number() {
    local tx=$1
    local block_number=$(cast receipt --rpc-url $rpc_url $tx blockNumber)
    echo $block_number
}

sum=0
count=0

process_gas() {
    local output=$1
    local gas=$(echo "$output" | grep "^cumulativeGasUsed" | awk '{print $2}')
    
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
modules=("tick" "order" "matcher" "engine" "bitmap" "mock-usdc" "mock-weth" "balance-manager" "pool-manager" "pool-orderbook")

# Define the deployment command
deploy_command="cargo stylus deploy -e \$rpc_url --private-key \$private_key --no-verify"

# Declare associative arrays to store addresses, txs, and block numbers
declare -A addresses
declare -A txs
declare -A blocks

# Loop through each module and deploy
for module in "${modules[@]}"; do
  echo "Deploying $module..."
  output=$(cd "$(dirname "$0")/../../$module" && eval $deploy_command)
  
  # Parse the deployed contract address and remove color codes
  address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployed code at address:/ {print $NF}')

  # Parse the deployed contract tx and remove color codes
  tx=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployment tx hash:/ {print $NF}')

  # Get block number for the transaction
  block=$(get_block_number $tx)

  # Store in the associative arrays
  addresses[$module]=$address
  txs[$module]=$tx
  blocks[$module]=$block
  
  echo "Deployed $module at address: ${addresses[$module]}"
  echo "Transaction hash: ${txs[$module]}"
  echo "Block number: ${blocks[$module]}"
done

# Update .env file with addresses, txs, and block numbers
for module in "${!addresses[@]}"; do
  snake_case_module=$(echo "$module" | sed -r 's/([a-z])([A-Z])/\1_\2/g; s/-/_/g' | tr '[:lower:]' '[:upper:]')
  
  # Update ADDRESS
  if ! grep -q "${snake_case_module}_ADDRESS=" ../../.env; then
    echo "${snake_case_module}_ADDRESS=${addresses[$module]}" >> ../../.env
  else
    sed -i "s/${snake_case_module}_ADDRESS=.*/${snake_case_module}_ADDRESS=${addresses[$module]}/" ../../.env
  fi

  # Update TX
  if ! grep -q "${snake_case_module}_TX=" ../../.env; then
    echo "${snake_case_module}_TX=${txs[$module]}" >> ../../.env
  else
    sed -i "s/${snake_case_module}_TX=.*/${snake_case_module}_TX=${txs[$module]}/" ../../.env
  fi

  # Update BLOCK
  if ! grep -q "${snake_case_module}_BLOCK=" ../../.env; then
    echo "${snake_case_module}_BLOCK=${blocks[$module]}" >> ../../.env
  else
    sed -i "s/${snake_case_module}_BLOCK=.*/${snake_case_module}_BLOCK=${blocks[$module]}/" ../../.env
  fi
done

echo "Total gas used: $sum"
[ $count -gt 0 ] && echo "Average gas used: $((sum / count))" || echo "No gas usage found"
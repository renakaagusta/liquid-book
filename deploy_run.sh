#!/bin/bash

# set -x

# private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
# rpc_url="http://localhost:8547"

# private_key="0x771026f019c25b4dcd7ed412d64c963dea19f9f6319c494432201fd17d8dbd38"
# rpc_url="https://arb-sepolia.g.alchemy.com/v2/jBG4sMyhez7V13jNTeQKfVfgNa54nCmF"

source .env

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
modules=("tick" "order" "matcher" "engine" "bitmap" "mock-usdc" "mock-weth" "balance-manager" "pool-manager" "pool-orderbook")
# modules=("balance-manager" "pool-manager" "pool-orderbook")

# Define the deployment command
deploy_command="cargo stylus deploy -e \$rpc_url --private-key \$private_key --no-verify"

# Declare an associative array to store addresses
declare -A addresses

# Declare an associative array to txs
declare -A txs

# Loop through each module and deploy
for module in "${modules[@]}"; do
  echo "Deploying $module..."
  output=$(cd "$(dirname "$0")/$module" && eval $deploy_command ) #2>&1)
  
  # Parse the deployed contract address and remove color codes
  address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployed code at address:/ {print $NF}')

  # Parse the deployed contract address and remove color codes
  tx=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployment tx hash:/ {print $NF}')

  # echo "$output"

  # Store the address in the associative array
  addresses[$module]=$address

  # Store the tx in the associative array
  txs[$module]=$tx
  
  echo "Deployed $module at address: ${addresses[$module]} on tx: ${txs[$module]}"

  # echo "Verifying $module..."
  # (cd "$(dirname "$0")/$module" && cargo stylus verify --deployment-tx=${txs[$module]})
done

for module in "${!addresses[@]}"; do
  snake_case_module=$(echo "$module" | sed -r 's/([a-z])([A-Z])/\1_\2/g; s/-/_/g' | tr '[:lower:]' '[:upper:]')
  if ! grep -q "${snake_case_module}_ADDRESS=" .env; then
    echo "export \"${snake_case_module}_ADDRESS=${addresses[$module]}\""
    echo "${snake_case_module}_ADDRESS=${addresses[$module]}" >> .env
  else
    sed -i "s/${snake_case_module}_ADDRESS=.*/${snake_case_module}_ADDRESS=${addresses[$module]}/" .env
  fi

  if ! grep -q "${snake_case_module}_TX=" .env; then
    echo "export \"${snake_case_module}_TX=${txs[$module]}\""
    echo "${snake_case_module}_TX=${txs[$module]}" >> .env
  else
    sed -i "s/${snake_case_module}_TX=.*/${snake_case_module}_TX=${txs[$module]}/" .env
  fi
done

source .env

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

# echo "Set initial tick"
# cast send --rpc-url $rpc_url --private-key $private_key $BITMAP_ADDRESS "setCurrentTick(int128)" 219800

echo "Add new pool"
cast send $POOL_MANAGER_ADDRESS "addPool(address,address,address,address,address,int128,uint256)" $POOL_ORDERBOOK_ADDRESS $MOCK_WETH_ADDRESS $MOCK_USDC_ADDRESS $ENGINE_ADDRESS $BITMAP_ADDRESS 219800 1000000000000000 --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1


echo "Total gas used: $sum"
[ $count -gt 0 ] && echo "Average gas used: $((sum / count))" || echo "No gas usage found"
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

echo "Place buy orders..."
# Place buy orders (true = buy)
for tick in $(seq 219780 -20 219280); do
  output=$(cast send --rpc-url $rpc_url --private-key $USER_2_PK $POOL_MANAGER_ADDRESS "placeOrder(address,uint256,int128,bool,bool)" $POOL_ORDERBOOK_ADDRESS 10000000000000000 $tick true false 2>&1)
  process_gas "$output"
done

echo "Place sell orders..."
# Place sell orders (false = sell)
for tick in $(seq 220300 -20 219800); do
  output=$(cast send --rpc-url $rpc_url --private-key $USER_3_PK $POOL_MANAGER_ADDRESS "placeOrder(address,uint256,int128,bool,bool)" $POOL_ORDERBOOK_ADDRESS 10000000000000000 $tick false false 2>&1)
  process_gas "$output"
done

# Check balances for USER_2 after deposit
echo "Checking USER_2 balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_2_ADDRESS $MOCK_USDC_ADDRESS

# Check balances for USER_3 after deposit
echo "Checking USER_3 balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_3_ADDRESS $MOCK_WETH_ADDRESS

# Check locked balances for USER_2 after deposit
echo "Checking USER_2 locked balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_2_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_USDC_ADDRESS

# Check locked balances for USER_3 after deposit
echo "Checking USER_3 locked balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_3_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_WETH_ADDRESS
source .env
source .env.example

# 219772

echo "Place buy orders..."
# Place buy orders (true = buy)
# for tick in {219771..219766}; do
for tick in {219771..219770}; do
  cast send --rpc-url $RPC_URL --private-key $USER_2_PK $POOL_MANAGER_ADDRESS  "placeOrder(address,uint256,int128,bool,bool)" $POOL_ORDERBOOK_ADDRESS 100000000000000000 $tick true false
done

echo "Place sell orders..."
# Place sell orders (false = sell)
# for tick in {219778..219773}; do
for tick in {219778..219777}; do
  cast send --rpc-url $RPC_URL --private-key $USER_3_PK $POOL_MANAGER_ADDRESS "placeOrder(address,uint256,int128,bool,bool)" $POOL_ORDERBOOK_ADDRESS 100000000000000000 $tick false false 
done

# Check balances for USER_2 after deposit
echo "Checking USER_2 balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_2_ADDRESS $MOCK_USDC_ADDRESS

# Check balances for USER_3 after deposit
echo "Checking USER_3 balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_3_ADDRESS $MOCK_WETH_ADDRESS

# Check locked balances for USER_2 after deposit
echo "Checking USER_2 locked balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_2_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_USDC_ADDRESS

# Check locked balances for USER_3 after deposit
echo "Checking USER_3 locked balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_3_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_WETH_ADDRESS
source .env

# echo "Market orders..."
# cast send --rpc-url $RPC_URL --private-key $USER_2_PK $POOL_MANAGER_ADDRESS  "placeOrder(address,uint256,int128,bool,bool)" $POOL_ORDERBOOK_ADDRESS 100000000000000000 219772 true true

# 600000000000000000

cast send --rpc-url $RPC_URL --private-key $USER_3_PK $POOL_MANAGER_ADDRESS "placeOrder(address,uint256,int128,bool,bool)" $POOL_ORDERBOOK_ADDRESS 10000000000000000 219772 false true

# Check balances for USER_2 after deposit
echo "Checking USER_2 balance USDC after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_2_ADDRESS $MOCK_USDC_ADDRESS

# Check balances for USER_2 after deposit
echo "Checking USER_2 balance WETH after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_2_ADDRESS $MOCK_WETH_ADDRESS

# Check balances for USER_3 after deposit
echo "Checking USER_3 balance WETH after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_3_ADDRESS $MOCK_WETH_ADDRESS

# Check balances for USER_3 after deposit
echo "Checking USER_3 balance USDC after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_3_ADDRESS $MOCK_USDC_ADDRESS

# Check locked balances for USER_2 after deposit
echo "Checking USER_2 locked balance USDC after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_2_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_USDC_ADDRESS

# Check locked balances for USER_2 after deposit
echo "Checking USER_2 locked balance WETH after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_2_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_WETH_ADDRESS

# Check locked balances for USER_3 after deposit
echo "Checking USER_3 locked balance WETH after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_3_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_WETH_ADDRESS

# Check locked balances for USER_3 after deposit
echo "Checking USER_3 locked balance USDC after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_3_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_USDC_ADDRESS

# Checking USER_2 balance after deposit...
# 97911063768 [9.791e10]
# Checking USER_3 balance after deposit...
# 99400000000000000000 [9.94e19]
# Checking USER_2 locked balance after deposit...
# 2088936232 [2.088e9]
# Checking USER_3 locked balance after deposit...
# 600000000000000000 [6e17]

# Checking USER_2 balance after deposit...
# 97562785865 [9.756e10]
# Checking USER_3 balance after deposit...
# 99300000000000000000 [9.93e19]
# Checking USER_2 locked balance after deposit...
# 2437214135 [2.437e9]
# Checking USER_3 locked balance after deposit...
# 700000000000000000 [7e17]
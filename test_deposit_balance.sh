source .env


# # Send ETH to USER_2 for gas
# cast send --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK $USER_2_ADDRESS --value 10000000000000000000

# # Send ETH to USER_3 for gas
# cast send --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK $USER_3_ADDRESS --value 10000000000000000000000000

# Mint tokens mock-usdc for USER_2 (100K USDC)
cast send --rpc-url $RPC_URL --private-key $USER_2_PK $MOCK_USDC_ADDRESS "mint(uint256)" 1000000000000

# Mint tokens mock-weth for USER_3 (100 ETH)
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $MOCK_WETH_ADDRESS "mint(uint256)" 1000000000000000000000

# Approve allowance for BalanceManager contract for USER_2
cast send --rpc-url $RPC_URL --private-key $USER_2_PK $MOCK_USDC_ADDRESS "approve(address,uint256)" $BALANCE_MANAGER_ADDRESS 100000000000

# Approve allowance for BalanceManager contract for USER_3
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $MOCK_WETH_ADDRESS "approve(address,uint256)" $BALANCE_MANAGER_ADDRESS 100000000000000000000

# Set operator for USER_2
echo "Setting operator for USER_2..."
cast send --rpc-url $RPC_URL --private-key $USER_2_PK $BALANCE_MANAGER_ADDRESS "setOperator(address,bool)" $POOL_MANAGER_ADDRESS true

# Set operator for USER_3
echo "Setting operator for USER_3..."
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $BALANCE_MANAGER_ADDRESS "setOperator(address,bool)" $POOL_MANAGER_ADDRESS true

# Deposit all USDC for USER_2 to the BalanceManager contract
cast send --rpc-url $RPC_URL --private-key $USER_2_PK $BALANCE_MANAGER_ADDRESS "deposit(address,uint256)" $MOCK_USDC_ADDRESS 100000000000

# Deposit all WETH for USER_3 to the BalanceManager contract
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $BALANCE_MANAGER_ADDRESS "deposit(address,uint256)" $MOCK_WETH_ADDRESS 100000000000000000000

# Check balances for USER_2 after deposit
echo "Checking USER_2 balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_2_ADDRESS $MOCK_USDC_ADDRESS

# Check balances for USER_3 after deposit
echo "Checking USER_3 balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_3_ADDRESS $MOCK_WETH_ADDRESS

# Check locked balances for USER_2 after deposit
echo "Checking USER_2 locked balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_2_ADDRESS $MOCK_USDC_ADDRESS $USER_2_ADDRESS

# Check locked balances for USER_3 after deposit
echo "Checking USER_3 locked balance after deposit..."
cast call --rpc-url $RPC_URL $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_3_ADDRESS $MOCK_WETH_ADDRESS $USER_3_ADDRESS


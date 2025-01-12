source .env

# Transfer locked tokens to new user address
echo "Transferring locked tokens to new user address..."
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $POOL_ORDERBOOK_ADDRESS "transferLocked(uint256,int128,address,address,bool)" 10000000000000000 219772 $USER_3_ADDRESS $USER_2_ADDRESS false

echo "Transferring locked tokens to new user address..."
cast send --rpc-url $RPC_URL --private-key $USER_2_PK $POOL_ORDERBOOK_ADDRESS "transferLocked(uint256,int128,address,address,bool)" 10000000000000000 219772 $USER_2_ADDRESS $USER_3_ADDRESS true

# POOL MANAGER
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $POOL_MANAGER_ADDRESS "transferLockedFromPool(uint256,address,address,address)" 100000000000000000 $MOCK_WETH_ADDRESS $USER_3_ADDRESS $USER_2_ADDRESS 

# BALANCE MANAGER
echo "Transferring locked tokens to new user address..."
cast send --rpc-url $RPC_URL --private-key $USER_3_PK $BALANCE_MANAGER_ADDRESS "transferLocked(address,address,address,address,uint256)" $USER_3_ADDRESS $POOL_MANAGER_ADDRESS  $USER_2_ADDRESS $MOCK_WETH_ADDRESS 10000000000000000

# Checking USER_2 balance after deposit...
# 95473849633 [9.547e10]
# Checking USER_3 balance after deposit...
# 98700000000000000000 [9.87e19]
# Checking USER_2 locked balance after deposit...
# 3829524905 [3.829e9]
# Checking USER_3 locked balance after 

# Checking USER_2 balance after deposit...
# 95125571730 [9.512e10]
# Checking USER_3 balance after deposit...
# 98600000000000000000 [9.86e19]
# Checking USER_2 locked balance after deposit...
# 4177802808 [4.177e9]
# Checking USER_3 locked balance after deposit...
# 1400000000000000000 [1.4e18]
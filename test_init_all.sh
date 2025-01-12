source .env
source .env.example

# Initialize engine
cast send $ENGINE_ADDRESS "initialize(address,address,address,address)" $TICK_ADDRESS $ORDER_ADDRESS $BITMAP_ADDRESS $MATCHER_ADDRESS --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK

# Initialize matcher
cast send $MATCHER_ADDRESS "initialize(address,address,address)" $BITMAP_ADDRESS $ORDER_ADDRESS $POOL_ORDERBOOK_ADDRESS --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK

# Initialize tick
cast send $TICK_ADDRESS "initialize(address,address,address)" $ENGINE_ADDRESS $BITMAP_ADDRESS $ORDER_ADDRESS --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK

# Initialize order
cast send $ORDER_ADDRESS "initialize(address,address,address)" $ENGINE_ADDRESS $BITMAP_ADDRESS $TICK_ADDRESS --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK 

# Initialize order
cast send $POOL_MANAGER_ADDRESS "initialize(address)" $BALANCE_MANAGER_ADDRESS --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK 

echo "Set initial tick"
cast send --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK $BITMAP_ADDRESS "setCurrentTick(int128)" 219772

echo "Add new pool"
cast send $POOL_MANAGER_ADDRESS "addPool(address,address,address,address,address,int128,uint256)" $POOL_ORDERBOOK_ADDRESS $MOCK_WETH_ADDRESS $MOCK_USDC_ADDRESS $ENGINE_ADDRESS $BITMAP_ADDRESS 219772 1000000000000000 --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK



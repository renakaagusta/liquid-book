source .env


# export RPC_URL=
export PRIVATE_KEY=$STYLUS_LOCAL_DEV_PK

# export ENGINE_ADDRESS=0xf7b97478528a26a2bce77d2b34a5bb078adbaaf3
# export MATCHER_ADDRESS=0xe6ae209ea8974ce6a1996c10471f14bdc8ddbd41
# export BITMAP_ADDRESS=0xf5277eb468001416ea15557096f7d0ff28cbfd94
# export TICK_ADDRESS=0x36c6f2442adc8993abd0e97f47e5711fc30633b5
# export ORDER_ADDRESS=0x4149e31e3498032030cb26b872b0d4fec9734877


# Initialize engine
cast send $ENGINE_ADDRESS "initialize(address,address,address,address)" $TICK_ADDRESS $ORDER_ADDRESS $BITMAP_ADDRESS $MATCHER_ADDRESS --rpc-url $RPC_URL --private-key $PRIVATE_KEY

# Initialize matcher
cast send $MATCHER_ADDRESS "initialize(address,address)" $BITMAP_ADDRESS $ORDER_ADDRESS --rpc-url $RPC_URL --private-key $PRIVATE_KEY

# Initialize tick
cast send $TICK_ADDRESS "initialize(address,address,address)" $ENGINE_ADDRESS $BITMAP_ADDRESS $ORDER_ADDRESS --rpc-url $RPC_URL --private-key $PRIVATE_KEY

# Initialize order
cast send $ORDER_ADDRESS "initialize(address,address,address)" $ENGINE_ADDRESS $BITMAP_ADDRESS $TICK_ADDRESS --rpc-url $RPC_URL --private-key $PRIVATE_KEY 

echo "Set initial tick"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "setCurrentTick(uint256)" 100

echo "Place sell orders..."
# Place sell orders (false = sell)
for tick in {105..100}; do
  cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $ENGINE_ADDRESS "placeOrder(uint256,uint256,address,bool,bool)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 
done

echo "Place buy orders..."
# Place buy orders (true = buy)
for tick in {99..95}; do
  cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $ENGINE_ADDRESS "placeOrder(uint256,uint256,address,bool,bool)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false
done

echo "Get top best ticks for seller"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "topNBestTicks(bool)" false 

echo "Get top best ticks for buyer"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "topNBestTicks(bool)" true 

echo "Get bitmap #1"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "getBitmap(int16)" 0

echo "Get tick #1" 
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "getCurrentTick()(uint256)" 

echo "Place limit buy order to fill sell order at tick 101"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $ENGINE_ADDRESS "placeOrder(uint256,uint256,address,bool,bool)" 101 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false

echo "Get bitmap #2"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "getBitmap(int16)" 0 

echo "Get tick #2"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "getCurrentTick()(uint256)" 

echo "Place limit sell order to fill buy order at tick 98 (The order book will use 100 to fill this order since it was the best buy tick)"
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $ENGINE_ADDRESS "placeOrder(uint256,uint256,address,bool,bool)" 98 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false

echo "Get bitmap #3"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "getBitmap(int16)" 0 

echo "Get tick #3"
cast call --rpc-url $RPC_URL --private-key $PRIVATE_KEY $BITMAP_ADDRESS "getCurrentTick()(uint256)" 
source .env


echo "Get current tick"
cast call --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK $BITMAP_ADDRESS "getCurrentTick()"

echo "Get top N best ticks for buyer"
cast call --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK $BITMAP_ADDRESS "topNBestTicks(bool)" true

echo "Get top N best ticks for seller"
cast call --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK $BITMAP_ADDRESS "topNBestTicks(bool)" false


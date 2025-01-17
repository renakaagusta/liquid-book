#!/bin/bash

LOG_FILE="$(dirname "$0")/trading_logs_4.txt"

log_message() {
    echo "$1" | tee -a "$LOG_FILE"
}

source "$(dirname "$0")/../.env"

private_key="$STYLUS_DEV_4_PK"
rpc_url="$RPC_DEV_URL"

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
        log_message "Gas used: $gas"
    fi
}

current_tick=$(echo "$(cast call --rpc-url $rpc_url --private-key $private_key $BITMAP_ADDRESS "getCurrentTick()(int128)")" | awk '{print $1}')

MIN=1000000
MAX=6000000
range=$(($MAX - $MIN + 1))
size=$(($MIN + ($RANDOM * 32768 + $RANDOM) % $range))

MIN=78000
MAX=84000
tick=$((RANDOM % ($MAX - $MIN + 1) + $MIN))

is_market="false"

is_buy="true"

export TZ=Asia/Jakarta

TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S %Z')
UTC_TIMESTAMP=$(TZ=UTC date '+%Y-%m-%d %H:%M:%S %Z')

log_message "=== Order Variables Log ==="
log_message "Jakarta Time: $TIMESTAMP"
log_message "UTC Time: $UTC_TIMESTAMP"
log_message "User Address: $USER_ADDRESS"
log_message "Tick: $tick"
log_message "Size: $size"
log_message "Is Buy Order: $is_buy"
log_message "Is Market Order: $is_market"
log_message "Current tick: $current_tick"

output=$(cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key $ENGINE_ADDRESS "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick $size $USER_ADDRESS $is_buy $is_market 2>&1)
process_gas "$output"

log_message "================================================="
#!/bin/bash

# set -x

private_key="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
rpc_url="http://localhost:8547"

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
<<<<<<< HEAD
<<<<<<< HEAD
modules=("tick" "order" "matcher" "engine" "bitmap")
=======
<<<<<<< HEAD
modules=("order" "matcher" "engine" "bitmap" "tick")
=======
modules=("bitmap" "tick" "order" "matcher" "engine" "pool-orderbook")
>>>>>>> 5a8fc53 (adjust matcher with balance-mamager)
>>>>>>> 9b38baa (adjust matcher with balance-mamager)
=======
modules=("order" "matcher" "engine" "bitmap" "tick")
>>>>>>> 8212104 (fix(pool-manager): fix size issue)

# Define the deployment command
deploy_command="cargo stylus deploy -e \$rpc_url --private-key \$private_key --no-verify"

# Declare an associative array to store addresses
declare -A addresses

# Loop through each module and deploy
for module in "${modules[@]}"; do
  echo "Deploying $module..."
  output=$(cd "$(dirname "$0")/$module" && eval $deploy_command 2>&1)
  
  # Parse the deployed contract address and remove color codes
  address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployed code at address:/ {print $NF}')

  # Store the address in the associative array
  addresses[$module]=$address
  
  echo "Deployed $module at address: ${addresses[$module]}"
done

# Print all addresses and export them to .env.example
echo "All deployed addresses:"
for module in "${!addresses[@]}"; do
  snake_case_module=$(echo "$module" | sed -r 's/([a-z])([A-Z])/\1_\2/g; s/-/_/g' | tr '[:lower:]' '[:upper:]')
  if ! grep -q "${snake_case_module}_ADDRESS=" .env.example; then
    echo "export \"${snake_case_module}_ADDRESS=${addresses[$module]}\""
    echo "${snake_case_module}_ADDRESS=${addresses[$module]}" >> .env.example
  else
    sed -i "s/${snake_case_module}_ADDRESS=.*/${snake_case_module}_ADDRESS=${addresses[$module]}/" .env.example
  fi
done

# echo "Initialize contracts"

# Initialize engine
<<<<<<< HEAD
cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize matcher
cast send "${addresses[matcher]}" "initialize(address,address,address)" "${addresses[tick]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize tick
cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# Initialize order
cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1 

echo "Set initial tick"
cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(int128)" 100 > /dev/null 2>&1

echo "Place sell orders..."
for tick in {110..100}; do
  output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 2>&1)
  process_gas "$output"
  cast call  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
done

echo "Place buy orders..."
for tick in {99..90}; do
  output=$(cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
  process_gas "$output"
  cast call  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
done
=======
# cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key
# cast send "${addresses[engine]}" "initialize(address,address,address,address)" "${addresses[tick]}" "${addresses[order]}" "${addresses[bitmap]}" "${addresses[matcher]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# # Initialize matcher
# # cast send "${addresses[matcher]}" "initialize(address,address)" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key 
# cast send "${addresses[matcher]}" "initialize(address,address)" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# # Initialize tick
# # cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key 
# cast send "${addresses[tick]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[order]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1

# # Initialize order
# # cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key 
# cast send "${addresses[order]}" "initialize(address,address,address)" "${addresses[engine]}" "${addresses[bitmap]}" "${addresses[tick]}" --rpc-url $rpc_url --private-key $private_key > /dev/null 2>&1 

# echo "Set initial tick"
# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(int128)" 100 > /dev/null 2>&1
# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "setCurrentTick(uint256)" 100  > /dev/null 2>&1

# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 100 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 

# echo "Place sell orders..."
# # Place sell orders (false = sell)
# for tick in {105..100}; do
#   # cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(uint256,uint256,address,bool,bool)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false
#   cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false
# done

# echo "Place buy orders..."
# # Place buy orders (true = buy)
# for tick in {99..95}; do
#   # cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(uint256,uint256,address,bool,bool)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 
#   cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false
# done
>>>>>>> 8212104 (fix(pool-manager): fix size issue)

# echo "Get top best ticks for seller"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" false 

# echo "Get top best ticks for buyer"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true 

# echo "Get bitmap #1"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0

# echo "Get tick #1" 
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 

<<<<<<< HEAD
echo "Place limit buy order to fill sell order at tick 101"
output=$(cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 101 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
process_gas "$output"

echo "Get bitmap #2"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1

echo "Get tick #2"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1

echo "Place limit sell order to fill buy order at tick 98 (The order book will use 100 to fill this order since it was the best buy tick)" > /dev/null 2>&1
output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 98 1000000000 "0xF63fb6da9b0EEdD4786C8ee464962b5E1b17AD1d" false false 2>&1)
process_gas "$output"

echo "Place sell orders..."
for tick in {99..90}; do
  # cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "log(int32)" $tick > /dev/null 2>&1
  output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 2>&1)
  process_gas "$output"
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true > /dev/null 2>&1
done

echo "Place buy orders from 90 to 96"
for tick in {90..96}; do
  # cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "log(int32)" $tick > /dev/null 2>&1
  output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
  process_gas "$output"
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true > /dev/null 2>&1
done

echo "Get bitmap #3"
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1

echo "Get tick #3"
<<<<<<< HEAD
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1

echo "Place additional limit buy order at 96"
output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 96 2000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
process_gas "$output"

echo "Place additional limit buy order at 96"
output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 96 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false 2>&1)
process_gas "$output"

echo "Place sell orders to fill orders at 96 to 95"
for tick in {93..90}; do
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true > /dev/null 2>&1
  output=$(cast send  --gas-limit 8000000 --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" $tick 1000000000 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" false false 2>&1)
  process_gas "$output"
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" > /dev/null 2>&1
  cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "topNBestTicks(bool)" true > /dev/null 2>&1
done

echo "Total gas used: $sum"
[ $count -gt 0 ] && echo "Average gas used: $((sum / count))" || echo "No gas usage found"
=======
cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 
=======
# echo "Place limit buy order to fill sell order at tick 101"
# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" 101 100 "0xf7ced2890a5428cc1f46252b0f04600234e8ab6e" true false

# echo "Get bitmap #2"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 

# echo "Get tick #2"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 

# echo "Place limit sell order to fill buy order at tick 98 (The order book will use 100 to fill this order since it was the best buy tick)"
# cast send --rpc-url $rpc_url --private-key $private_key "${addresses[engine]}" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" -- -100 2577619610815930 "0xF63fb6da9b0EEdD4786C8ee464962b5E1b17AD1d" true false

# echo "Get bitmap #3"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getBitmap(int16)" 0 

# echo "Get tick #3"
# cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "getCurrentTick()(int128)" 
>>>>>>> 8212104 (fix(pool-manager): fix size issue)

echo "Get conversion from tick to price"
price_hex=$(cast call --rpc-url $rpc_url --private-key $private_key "${addresses[bitmap]}" "convertFromTickToPrice(int128)" 219772)
price_dec=$(printf "%d\n" "$price_hex")
echo "Price: $price_dec"
>>>>>>> 9b38baa (adjust matcher with balance-mamager)

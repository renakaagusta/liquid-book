#!/bin/bash

# set -x

private_key="45d37ea082249aa1349f24663fbcfdc325b4bce530527e929c4356fc925f4f47"
rpc_url="https://arb-sepolia.g.alchemy.com/v2/jBG4sMyhez7V13jNTeQKfVfgNa54nCmF"

echo "Place limit sell order to fill buy order at tick 98 (The order book will use 100 to fill this order since it was the best buy tick)"
cast send --rpc-url $rpc_url --private-key $private_key "0xab163eb7b64fc0478844b29321cc7af85676f7f7" "placeOrder(int128,uint256,address,bool,bool)(uint256,i128,u256)" "-- -100" 2577619610815930 "0xF63fb6da9b0EEdD4786C8ee464962b5E1b17AD1d" true false
#!/bin/bash

source "$(dirname "$0")/../../.env"

# Check if the script is run with "dev" argument
if [ "$1" == "dev" ]; then
  private_key="$STYLUS_DEV_PK"
  rpc_url="$RPC_DEV_URL"
else
  private_key="$STYLUS_LOCAL_DEV_PK"
  rpc_url="$RPC_URL"
fi

# USER 2 as a seller

# Mint tokens mock-weth for user 2 (100 ETH)
echo "Minting WETH..."
cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $USER_2_PK $MOCK_WETH_ADDRESS "mint(uint256)" 100000000000000000000 > /dev/null 2>&1

# Check tokens balance mock-weth for user 2 (100 ETH)
echo "Checking balance..."
cast call --rpc-url $rpc_url --private-key $USER_2_PK $MOCK_WETH_ADDRESS "balanceOf(address)" $USER_2_ADDRESS

# Approve allowance for BalanceManager contract for USER
echo "Approving allowance..."
cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $USER_2_PK $MOCK_WETH_ADDRESS "approve(address,uint256)" $BALANCE_MANAGER_ADDRESS 100000000000000000000 > /dev/null 2>&1

# Deposit all WETH for USER to the BalanceManager contract
echo "Deposit WETH..."
cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $USER_2_PK $BALANCE_MANAGER_ADDRESS "deposit(address,uint256)" $MOCK_WETH_ADDRESS 100000000000000000000 > /dev/null 2>&1

# Check balances for USER after deposit
echo "Checking balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_2_ADDRESS $MOCK_WETH_ADDRESS

# Check locked balances for USER after deposit
echo "Checking locked balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_2_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_WETH_ADDRESS

# User 3 as a buyer

echo "$USER_3_PK $USER_3_ADDRESS"

# Mint tokens mock-usdc for USER (100 ETH)
echo "Minting USDC..."
cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $USER_3_PK $MOCK_USDC_ADDRESS "mint(uint256)" 100000000000000000000000 > /dev/null 2>&1

# Check tokens balance mock-weth for USER (100 ETH)
echo "Checking balance..."
cast call --rpc-url $rpc_url --private-key $USER_3_PK $MOCK_USDC_ADDRESS "balanceOf(address)" $USER_3_ADDRESS

# Approve allowance for BalanceManager contract for USER
echo "Approving allowance..."
cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $USER_3_PK $MOCK_USDC_ADDRESS "approve(address,uint256)" $BALANCE_MANAGER_ADDRESS 100000000000000000000000 > /dev/null 2>&1

# Deposit all USDC for USER to the BalanceManager contract
echo "Deposit USDC..."
cast send --gas-limit 8000000 --rpc-url $rpc_url --private-key $USER_3_PK $BALANCE_MANAGER_ADDRESS "deposit(address,uint256)" $MOCK_USDC_ADDRESS 100000000000000000000000 > /dev/null 2>&1

# Check balances for USER after deposit
echo "Checking balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getBalance(address,address)(uint256)" $USER_3_ADDRESS $MOCK_USDC_ADDRESS

# Check locked balances for USER after deposit
echo "Checking locked balance after deposit..."
cast call --rpc-url $rpc_url $BALANCE_MANAGER_ADDRESS "getLockedBalance(address,address,address)(uint256)" $USER_3_ADDRESS $POOL_MANAGER_ADDRESS $MOCK_USDC_ADDRESS

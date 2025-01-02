export RPC_URL="http://localhost:8547"
export STYLUS_LOCAL_DEV_PK="0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
export USER_ADDRESS="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"

original_dir=$(pwd)

cd lending

# Deploy the mock USDC contract
output=$(forge create src/mocks/MockUSDC.sol:MockUSDC --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK --broadcast)
mock_usdc_address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/Deployed to:/ {print $NF}')

# Deploy the Lending contract with the address of the mock USDC contract
output=$(forge create src/Lending.sol:Lending --rpc-url $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK --broadcast --constructor-args $mock_usdc_address)
lending_address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/Deployed to:/ {print $NF}')
export LENDING_ADDRESS=$lending_address

cd "$original_dir"

cd lending-execute
output=$(cargo stylus deploy -e $RPC_URL --private-key $STYLUS_LOCAL_DEV_PK --no-verify)
address=$(echo "$output" | sed -r 's/\x1b\[[0-9;]*m//g' | awk '/deployed code at address:/ {print $NF}')
export LENDING_EXECUTE_ADDRESS=$address

echo "export RPC_URL=http://localhost:8547"
echo "export STYLUS_LOCAL_DEV_PK=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659"
echo "export USER_ADDRESS=0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
echo "export MOCK_USDC_ADDRESS=$mock_usdc_address"
echo "export LENDING_ADDRESS=$lending_address"
echo "export LENDING_EXECUTE_ADDRESS=$address"

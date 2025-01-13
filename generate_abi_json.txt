# Loop through each .sol file in the contracts directory except matcher.sol
for sol_file in "$(dirname "$0")/contracts"/*.sol; do
  # Skip matcher.sol file
  if [[ "$(basename "$sol_file")" == "matcher.sol" ]]; then
    continue
  fi

  # Extract the base name of the file without extension
  base_name=$(basename "$sol_file" .sol)
  
  # Compile the .sol file to generate the ABI
  if ! command -v solc &> /dev/null; then
    echo "Error: solc command not found"
    continue
  fi

  solc --abi "$sol_file" -o "$(dirname "$0")/contracts"

  # Use regex to find the ABI file with the pattern <something><base_name><something>.abi
  abi_file_path=$(find "$(dirname "$0")/contracts" -type f -name "*${base_name}*.abi" | head -n 1)

  # Check if the ABI file exists
  if [ -z "$abi_file_path" ]; then
    echo "Error: No ABI file found for $base_name"
    continue
  fi

  # Read the ABI content
  abi_content=$(cat "$abi_file_path")

  # Print the ABI content and the file it was read from for testing
#   echo "Testing: ABI content from $abi_file_path"
#   echo "$abi_content"

  # Extract the ABI array from the content
  abi_array=$(echo "$abi_content" | sed -n '/^\[.*\]$/p')

  # Check if the ABI array is empty
  if [ -z "$abi_array" ]; then
    echo "Warning: ABI array is empty for $base_name"
    continue
  fi

  # Define the TypeScript and JSON file paths
  ts_file_path="$(dirname "$0")/contracts/${base_name}.ts"
  json_file_path="$(dirname "$0")/contracts/${base_name}.json"

  # Write the ABI array to the TypeScript file
  echo "export const $(echo "${base_name}" | sed -r 's/(^|-)([a-z])/\U\2/g') = ${abi_array};" > "$ts_file_path"

  # Write the ABI array to the JSON file
  echo "{ \"abi\": ${abi_array} }" > "$json_file_path"

  echo "Generated ${base_name}.ts and ${base_name}.json from ${abi_file_path}"

done

# Remove all .abi files inside the /contracts directory
find "$(dirname "$0")/contracts" -type f -name "*.abi" -exec rm -f {} \;

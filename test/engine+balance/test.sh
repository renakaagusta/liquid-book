#!/bin/bash

# Check if the script is run with "dev" argument
if [ "$1" == "dev" ]; then
    dev="true"
else
    dev="false"
fi

# Define scripts to run
scripts=("deploy.sh" "fund.sh" "initialize.sh" "deposit.sh" "limit_orders.sh")

# Function to run script and check status
run_script() {
    echo "Starting $1..."
    
    if [$dev == "true"]; then
        if ./"$1" dev; then
            echo "$1 completed successfully"
            return 0
        else
            echo "$1 failed with exit code $?"
            return 1
        fi
    else
        if ./"$1" dev; then
            echo "$1 completed successfully"
            return 0
        else
            echo "$1 failed with exit code $?"
            return 1
        fi
    fi
}

# Make scripts executable
for script in "${scripts[@]}"; do
    chmod +x "$script"
done

# Run each script
for script in "${scripts[@]}"; do
    echo "==== EXECUTING $script ====="
    if ! run_script "$script" "dev"; then
        echo "Error running $script. Stopping execution."
        exit 1
    fi
done

echo "All scripts completed successfully"
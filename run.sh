#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: ./run.sh [name]"
    exit 1
fi

# Get the name from the command line argument
name=$1

# Run the experiment with Cargo
echo "Running $name..."
RUSTFLAGS=-Awarnings cargo run --bin "$name"

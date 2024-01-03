#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: ./run.sh [name]"
    exit 1
fi

# Get the name from the command line argument
name=$1

# Clear old build
echo "Clearing old build..."
rm -f ./Output/"$name"

# Compile the Rust file
echo "Compiling $name.rs..."
rustc ./Experiments/"$name"/"$name".rs -o ./Output/"$name"

# Check if compilation was successful
if [ $? -eq 0 ]; then
    # Run the compiled build
    echo "Running $name..."
    printf "\n"
    ./Output/"$name"
else
    echo "Compilation failed. Please check the code for errors."
fi

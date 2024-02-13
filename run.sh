#!/bin/bash
#TODO: completions.
# Check if the correct number of arguments is provided
if [ "$#" -lt 1 ]; then
  echo "Usage: ./run.sh <name> [-v|--verbose]"
  exit 1
fi


# Get the name from the command line argument
name=$1

# If -d flag is provided, run in debug mode
if [ "$2" == "-v" ] || [ "$2" == "--verbose" ]; then
  printf -- "-----------\e[93m%s\e[0m-----------\n\n" "$name"
  cargo run --bin "$name"
else
  printf -- "-----------\e[93m%s\e[0m-----------\n\n" "$name"
  RUSTFLAGS=-Awarnings cargo run -q --bin "$name"
fi

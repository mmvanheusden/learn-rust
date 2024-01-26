# Creates new experiment
# Usage: ./newExperiment.sh <experiment name>

# Check if experiment name is given
if [ $# -eq 0 ]
  then
    echo "No experiment name supplied"
    exit 1
fi

# Create experiment folder + rust file
mkdir "./Experiments/$1"
touch "./Experiments/$1/$1.rs"

# Create a main function
echo "#![allow(non_snake_case)]
/// <WRITE DESCRIPTION HERE>
fn main() {
    println!(\"Hello, world!\");
}" >> "./Experiments/$1/$1.rs"

# Append to Cargo.toml
echo "
[[bin]]
name = \"$1\"
path = \"Experiments/$1/$1.rs\"" >> Cargo.toml

echo "Created new experiment: $1"
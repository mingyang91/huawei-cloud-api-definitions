#!/bin/bash

# Check if base version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 base_version"
    exit 1
fi

base_version=$1

# Function to update the version in Cargo.toml
update_version() {
    local crate_dir=$1
    local new_version=$2
    local cargo_file="$crate_dir/Cargo.toml"

    if [ -f "$cargo_file" ]; then
        # Use a tool like 'sed' to update the version in Cargo.toml
        sed -i "s/^version = \"CHANGE_ME\"/version = \"$new_version\"/" "$cargo_file"
    fi
}

# Main loop for each sub-crate
for crate_dir in schemas/* ; do
    echo "Processing $crate_dir"

    # Check for changes in the last day
    if git diff --quiet HEAD "HEAD@{1 day ago}" -- "$crate_dir"; then
        echo "No changes in $crate_dir"
        continue
    fi

    # Get the current date in the format yearmonthday (e.g., 20211231)
    current_date=$(date +"%Y%m%d")

    # Update the version in Cargo.toml (x.y.date format)
    update_version "$crate_dir" "$base_version$current_date"

    # Publish the crate (uncomment the following line if you have a publish command)
    cargo publish --manifest-path "$crate_dir/Cargo.toml"
done

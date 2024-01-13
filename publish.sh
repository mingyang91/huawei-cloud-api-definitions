#!/bin/bash

# Check if base version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 base_version"
    exit 1
fi

base_version=$1

check_crate_version_published() {
    local crate_name="$1"
    local target_version="$2"

    # Fetch the list of all versions of the crate from crates.io
    local crate_versions=$(curl -s "https://crates.io/api/v1/crates/$crate_name" | jq -r '.versions[].num')

    # Check if the target version is in the list
    if [[ $crate_versions == *"$target_version"* ]]; then
        echo "Version $target_version of $crate_name is already published."
        return 0 # Return 0 for true/success
    else
        echo "Version $target_version of $crate_name is not published."
        return 1 # Return 1 for false/failure
    fi
}

# Function to update the version in Cargo.toml
update_version() {
    local crate_dir=$1
    local new_version=$2
    local cargo_file="$crate_dir/Cargo.toml"

    if [ -f "$cargo_file" ]; then
        # Update the version in Cargo.toml
        sed -i "s/^version = \"CHANGE_ME\"/version = \"$new_version\"/" "$cargo_file"
    fi
}

# Main loop for each sub-crate
for crate_dir in schemas/* ; do
    echo "Processing $crate_dir"
    crate_name=$(basename "$crate_dir") # Extract just the directory name
    crate_name="huawei-cloud-api-definitions-$crate_name"

    # Check for changes in the last day
    if git diff --quiet HEAD "HEAD@{1 day ago}" -- "$crate_dir"; then
        echo "No changes in $crate_dir"
        continue
    fi

    # Get the current date in the format yearmonthday (e.g., 20211231)
    current_date=$(date +"%Y%m%d")
    target_version="$base_version$current_date"

    if check_crate_version_published "$crate_name" "$target_version"; then
        continue
    fi

    # Update the version in Cargo.toml
    update_version "$crate_dir" "$target_version"

    # Publish the crate
    cargo publish --manifest-path "$crate_dir/Cargo.toml"
done

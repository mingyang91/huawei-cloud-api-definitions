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

# Function to get the last modification timestamp of the most recently modified file in a directory
get_last_modification_timestamp() {
    local folder_path=$1
    local last_mod_time

    # Find the most recently modified file and get its modification time
    last_mod_time=$(find "$folder_path" -type f -printf '%T@ %p\n' | sort -n -r | head -n 1 | cut -d' ' -f1)

    echo "$last_mod_time"
}

# Main loop for each sub-crate
for crate_dir in schemas/* ; do
    echo "Processing $crate_dir"
    crate_name=$(basename "$crate_dir") # Extract just the directory name
    crate_name="huawei-cloud-api-definitions-$crate_name"

    # Get the last change date in the format yearmonthday (e.g., 20211231)
		last_mod_time=$(get_last_modification_timestamp "$crate_dir")
    last_change_date=$(date -d "@$last_mod_time" +"%Y%m%d")
    target_version="$base_version$last_change_date"

    if check_crate_version_published "$crate_name" "$target_version"; then
        continue
    fi

    # Update the version in Cargo.toml
    update_version "$crate_dir" "$target_version"

    # Publish the crate
    cargo publish --manifest-path "$crate_dir/Cargo.toml" --allow-dirty
done

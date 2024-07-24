#!/bin/bash

# Find all symbolic links
find . -type l | while read -r symlink; do
    # Get the target file path
    target=$(readlink "$symlink")
    
    # Check if the target is an absolute or relative path and resolve it
    if [[ "$target" == /* ]]; then
        absolute_target="$target"
    else
        absolute_target=$(dirname "$symlink")/"$target"
    fi

    # Copy the content of the target file to a new file
    cp "$absolute_target" "$symlink"

    echo "Converted $symlink to a regular file containing the content of $absolute_target"
done


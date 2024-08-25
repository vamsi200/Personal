#!/bin/bash

# Wrapper script to print and execute commands
while IFS= read -r line; do
    echo "$line"
    eval "$line"
done < "$1"

#! /usr/bin/bash

for file in *; do
  if [[ -f $file ]]; then
    extension="${file##*.}"
    echo "[INFO] Creating dir with name: '$extension'" | tee -a organize_script.log
    mkdir -p "$extension"
    echo "[INFO] Moving $file to '$extension'" | tee -a organize_script.log
    mv "$file" "$extension/"
  fi
done

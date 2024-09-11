#! /usr/bin/bash

if [ $# -lt 1 ]; then
    echo "Usage: ./file_search [extension] [path] [file_name]"
    exit 1
fi

mode=$1
ext=$2
path=${3:-$PWD}
file_name=$4

if [ "$mode" == "fx" ]; then
    if [ -n "$ext" ]; then
        GET_FILES_WITH_EXTENSION=$(find "$path" -type f -name "*.$ext")
        echo "$GET_FILES_WITH_EXTENSION"
    else
        echo "Error: Extension Not provided"
        exit 1
    fi

elif [ "$mode" == "fn" ]; then
   if [ -n "$2" ] || [ -n "$3" ]; then
      file_name=$3
      path=$2
      GET_FILES_WITH_NAME=$(find "$path" -type f -name "*$file_name*")
      echo "$GET_FILES_WITH_NAME"
  else
      echo "Error: File Name Not provided"
      exit 1
  fi

else
    echo "Error: No Arguments provided"
    exit 1
fi

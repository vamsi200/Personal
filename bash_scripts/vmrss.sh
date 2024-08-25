#! /usr/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: ./vmrss.sh PID"
  exit 1
else
  PID=$1
fi

GET_MEMORY_USAGE=$(cat /proc/$PID/status | grep "VmRSS" | awk '{printf "%.2f MB\n", $2/1024}')

echo "Memory::""$GET_MEMORY_USAGE"

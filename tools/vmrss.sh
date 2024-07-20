#!/bin/bash

check_vmrss() {
  if [ -z "$1" ]; then
    echo "Usage: check_vmrss PID"
    return 1
  else
    cat "/proc/$1/status" | grep VmRSS
  fi
}

check_vmrss "$1"


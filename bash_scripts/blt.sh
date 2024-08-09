#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: ./blt.sh d or c"
  echo "d for disconnect"
  echo "c for connect"
  exit 1
fi

DEVICE_MAC=$(bluetoothctl devices | awk '{print $2}')

if [ "$1" == "d" ]; then
  if [ -n "$DEVICE_MAC" ]; then
    echo -e "disconnect $DEVICE_MAC\nquit" | bluetoothctl
  else
    echo "[*] No Devices Detected"
  fi

elif [ "$1" == "c" ]; then
  if [ -n "$DEVICE_MAC" ]; then
    echo -e "connect $DEVICE_MAC\nquit" | bluetoothctl
  else
    echo "[*] No Devices Detected"
  fi

else
  echo "Usage: ./blt.sh d or c"
  echo "d for disconnect"
  echo "c for connect"
  exit 1
fi

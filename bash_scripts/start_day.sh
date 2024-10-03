#! /bin/bash

connect_bluetooth(){
  echo "[*] Connecting to bluetooth device.."
  bluetooth_connect=$($HOME/scripts/Personal/bash_scripts/blt.sh c)

}

start_firefox(){
  echo "[*] Starting Firefox - (Opening Gmail & Youtube).."
  open_firefox=$(firefox "https://gmail.com" "https://youtube.com")
}

start_chromium(){
  echo "[*] Starting Chromium - (Opening LMS & StudentZone).."
  open_chromium=$(chromium "https://lms.muonline.ac.in/" "https://www.muonline.ac.in/studentzone/")
}

connect_bluetooth
start_firefox
start_chromium



#!/bin/bash
export DISPLAY=:0
export XDG_RUNTIME_DIR=/run/user/1000

get_ports=$(nmap -sC -sV -p- -T4 "localhost" -oN "$HOME/temp/scan_$(date +'%Y-%m-%d').txt")
port=$(grep -w open $HOME/temp/scan* | grep -v '^|_')
zenity --info --text="$port" --width=350 --height=200

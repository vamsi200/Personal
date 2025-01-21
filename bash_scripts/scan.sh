export DISPLAY=:0
export XDG_RUNTIME_DIR=/run/user/1000

file="$HOME/temp/scan_$(date +'%Y-%m-%d').txt"

nmap_scan=$(nmap -sC -sV -p- -T4 "localhost" -oN "$file")

port=$(grep -w open $file | grep -v '^|_')

zenity --info --text="$port" --width=350 --height=200


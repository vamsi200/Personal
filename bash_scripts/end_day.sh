#! /usr/bin/zsh

stop_browser() {
    echo "[*] Stopping <Firefox and Chromium>"

    pid_firefox=$(pgrep firefox)
    if [ -n "$pid_firefox" ]; then
        kill -15 $pid_firefox && echo "[*] SIGTERM sent to <Firefox>" || echo "[!] Failed to send SIGTERM to <Firefox>"
    else
        echo "[!] No <Firefox> process found"
    fi

    pid_chromium=$(pgrep chromium)
    if [ -n "$pid_chromium" ]; then
        kill -15 $pid_chromium && echo "[*] SIGTERM sent to <Chromium>" || echo "[!] Failed to send SIGTERM to <Chromium>"
    else
        echo "[!] No <Chromium> process found"
    fi

    for pid in $pid_firefox $pid_chromium; do
        [ -n "$pid" ] && while ps -p "$pid" >/dev/null; do
            echo "[*] Process <$pid> is still running"
            sleep 5
        done
    done
    echo "[*] Stopped the <Procceses>"
}

disconnect_bluetooth() {
    echo "[*] Disconnecting <Bluetooth device>"
    bluetooth_disconnect=$($HOME/scripts/Personal/bash_scripts/blt.sh d)
    status=$?

    if [ $status -eq 0 ]; then
        echo "[*] Bluetooth Device <Disconnected>"
    fi
}

get_data_usage() {
    get_date=$(date +'%b-%d-%Y')
    echo -e "[*] Getting Data Usage - [$get_date]"
    data_usage=$(grep wlan0 /proc/net/dev | awk '{
    received_gb = $2 / (1024 * 1024 * 1024);
    transmitted_gb = $10 / (1024 * 1024 * 1024);
    printf "[Received: %.2f GB, Transmitted: %.2f GB]", received_gb, transmitted_gb
}')
    echo "$get_date - $data_usage" >> $HOME/storage/data_usage.txt
    echo "[*] Data <Saved>"

}

disconnect_bluetooth
stop_browser
get_data_usage

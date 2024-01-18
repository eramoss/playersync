#!/bin/bash

mkdir -p "$HOME/.config/playersync/.config/playersync"
touch "$HOME/.config/playersync/.config/playersync/dbus_monitor.log"
output_file="$HOME/.config/playersync/dbus_monitor_output.log"

pid_file="$HOME/.config/playersync/dbus_monitor_daemon.pid"

if [ -e "$pid_file" ] && ps -p $(cat "$pid_file") > /dev/null 2>&1; then
    echo "Daemon is already running. Exiting."
    exit 1
fi

dbus-monitor "type=signal,path=/org/mpris/MediaPlayer2" > "$output_file" 2>&1 &

pid=$!

echo $pid > "$pid_file"

echo "dbus-monitor daemon is running in the background. PID: $pid"

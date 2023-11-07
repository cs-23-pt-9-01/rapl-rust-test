#!/bin/bash

IP="192.168.0.5"
PORT=":5000"

#Send http request to raspberry to start or stop logging
#$1: "start" or "stop"
if [ "$1" == "start"  || "$1" == "stop" ]; then
    curl http://$IP$PORT/$1
else
    echo Bad input: "$1", expected "start" or "stop"
    exit 1
fi

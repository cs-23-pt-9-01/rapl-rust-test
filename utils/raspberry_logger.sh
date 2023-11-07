#!/bin/bash

IP="192.168.0.5"
PORT=":5000"

#Send http request to raspberry to start or stop logging
#$1: "start" or "stop"
curl http://$IP$PORT/$1

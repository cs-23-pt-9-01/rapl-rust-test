#!/bin/bash

for i in {1..5}; do
  time=$(date)
  echo starting run number $i at $time
  taskset -c 0,1 sh runBench.sh
  sleep 60s
done

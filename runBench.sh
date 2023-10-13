#!/bin/bash

##########################
##### Pre Benchmarks #####
##########################

#Send start signal to Raspberry PI - await confirmation from raspberry?
bash utils/raspberry_logger.sh 1

# Stop services and background processes
bash utils/kill_and_burn.sh 0

##########################
##### Run Benchmarks #####
########################## 
echo "Starting benchmarks"

# Create dir for results
mkdir results

# Running all benchmarks
for f in benchRunners/*.sh; do
  bash "$f"
done

###########################
##### Post Benchmarks #####
###########################

# starting services
bash utils/kill_and_burn.sh 1

# waiting for services to start
echo "waiting 10 secounds for services to start"
sleep 10s

# Send stop signal to Raspberry PI
bash utils/raspberry_logger.sh 0

# Send results data to Raspberry PI
bash utils/send_results.sh




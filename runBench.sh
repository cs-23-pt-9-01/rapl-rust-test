#!/bin/bash

# first argument is for stopping the logging from raspberry
# secound argument is for stopping the kill and burn script

arg1=${1:-'true'}
arg2=${2:-'true'}

##########################
##### Pre Benchmarks #####
##########################

#Send start signal to Raspberry PI - await confirmation from raspberry?
echo "starting logger"

if [ $arg1 != 'false' ]
then
bash utils/raspberry_logger.sh 1
sleep 10s
fi

if [ $arg2 != 'false' ]
then
# Stop services and background processes
bash utils/kill_and_burn.sh 0
fi

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

if [ $arg2 != 'false' ]
then
# starting services
bash utils/kill_and_burn.sh 1


# waiting for services to start
echo "waiting 10 secounds for services to start"
sleep 10s
fi

if [ $arg1 != 'false' ]
then
# Send stop signal to Raspberry PI
bash utils/raspberry_logger.sh 0
fi


if [ $arg2 != 'false' ]
then
# Send results data to Raspberry PI
bash utils/send_results.sh
fi



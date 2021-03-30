#!/bin/sh

# *******************************************************************************************************************************************************************************
# Author: Chinmay Kulkarni                                                                                                                                                      *
# University: The University of Georgia                                                                                                                                         *
# Script Description: A script to log the CPU, memmory usage of docker swarm functions running on raspberry pi 4 for fucntions - cosineflop, ddcmd, fastft, himatmul            *
#                     imageclassification, imgres, lowmatmul, medmatmul, netalex, nwperformance, objectclasss, sentiment-analysis, sineflop, sorter, speech-to-textsqrtflop     *
# Reference: http://www.zakariaamine.com/2019d-12-04/monitoring-docker                                                                                                           *
# *******************************************************************************************************************************************************************************

echo "Starting script to collect stats required for benchmarking..."
echo

echo "Trying to capture the container ID for sineflop function in the docker swarm..."
echo

# ************************************************************************************************************************
# To get the container ID for sineflop application and capture it in a variable.                                         *
# Ancestor based filter displays information related to container based on the image name                                * 
# ************************************************************************************************************************
SINEFLOP_CONTAINER_ID=$(sudo docker ps -f ancestor=cd21/sineflop:latest --format {{.ID}})

echo "SINEFLOP_CONTAINER_ID $SINEFLOP_CONTAINER_ID is captured."
echo

echo "Running docker stats command for sineflop function using $SINEFLOP_CONTAINER_ID container id..."
echo

# ************************************************************************************************************************
# This command is working and prints out the output in the resepective function_name.json file.                          *
# Reference: https://kylewbanks.com/blog/docker-stats-memory-cpu-in-json-format                                          *
# ************************************************************************************************************************
docker stats --no-stream --format "{\"Name\":\"SineFLOP\",\"container\":\"{{ .Container }}\",\"memory\":{\"raw\":\"{{ .MemUsage }}\",\"percent\":\"{{ .MemPerc }}\"},\"cpu\":\"{{ .CPUPerc }}\"}" | grep $SINEFLOP_CONTAINER_ID >> SINEFLOP_LOGS.JSON

echo "Please check the SINEFLOP_LOGS.JSON file."
echo
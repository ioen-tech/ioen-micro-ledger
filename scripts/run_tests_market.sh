#!/bin/bash

ITERATIONS=$1
DELAY=$2

sh ~/scripts/watch_system_usage.sh > ~/testresults/system_usage.txt & ID=$!

TIMESTAMP=$( date +%T )
echo $TIMESTAMP Idle >> ~/testresults/idle.txt

sleep 30

TIMESTAMP=$( date +%T )
echo $TIMESTAMP Begin requests >> ~/testresults/idle.txt

newman run ~/scripts/tem-Market.postman_collection.json -n $ITERATIONS --delay-request $DELAY > ~/testresults/test_results.txt

TIMESTAMP=$( date +%T )
echo $TIMESTAMP Idle >> ~/testresults/idle.txt

sleep 65

kill -9 $ID
docker-compose logs > ~/testresults/logs.txt

docker-compose down -v




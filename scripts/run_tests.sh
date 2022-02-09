#!/bin/bash

ITERATIONS=$1
DELAY=$2

sh ~/scripts/watch_system_usage.sh > ~/testresults/system_usage.txt & ID=$!
echo - >> ~/testresults/system_usage.txt
echo Idle >> ~/testresults/system_usage.txt
echo - >> ~/testresults/system_usage.txt

sleep 30

echo - >> ~/testresults/system_usage.txt
echo Begin requests >> ~/testresults/system_usage.txt
echo - >> ~/testresults/system_usage.txt

newman run ~/scripts/ledger-Agent.postman_collection.json -n $ITERATIONS --delay-request $DELAY > ~/testresults/test_results.txt
echo - >> ~/testresults/system_usage.txt
echo Idle >> ~/testresults/system_usage.txt
echo - >> ~/testresults/system_usage.txt

sleep 65

kill -9 $ID

docker-compose logs > ~/testresults/logs.txt

docker-compose down -v
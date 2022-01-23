#!/bin/bash

ITERATIONS=$1
DELAY=$2

MARKET_AGENT=118.138.243.49
NUMBER_OF_AGENTS=29
DT=$(date '+%d.%m.%Y_%H.%M.%S');
IFS=$'\n' read -d '' -r -a AGENT_LIST < ~/scripts/agent_list.txt

# Bring all docker containers up
for i in $(seq 0 $(($NUMBER_OF_AGENTS - 1)));
do
    AGENT=${AGENT_LIST[i]}
    ssh -i IOENTest.txt ubuntu@$AGENT "docker-compose up -d"
done
ssh -i ~/IOENTest.txt ubuntu@$MARKET_AGENT "docker-compose up -d"

sleep 30

TEST_FOLDER=${DT}-${NUMBER_OF_AGENTS}agents
echo $TEST_FOLDER

mkdir testresults
mkdir ~/testresults/$TEST_FOLDER
mkdir ~/testresults/$TEST_FOLDER/market

ssh -i IOENTest.txt ubuntu@$MARKET_AGENT "sh scripts/run_tests_market.sh $ITERATIONS $DELAY" && scp -r -i IOENTest.txt ubuntu@$MARKET_AGENT:~/testresults/* ~/testresults/$TEST_FOLDER/market && echo Tests for market finished &

for i in $(seq 0 $(($NUMBER_OF_AGENTS - 1)));
do
    AGENT=${AGENT_LIST[i]}
    echo Begin test for $AGENT
    mkdir ~/testresults/$TEST_FOLDER/$AGENT
    ssh -i IOENTest.txt ubuntu@$AGENT "sh scripts/run_tests.sh $ITERATIONS $DELAY" && scp -r -i IOENTest.txt ubuntu@$AGENT:~/testresults/* ~/testresults/$TEST_FOLDER/$AGENT && echo Tests for $AGENT finished &
done

# Clean testresults
for i in $(seq 0 $(($NUMBER_OF_AGENTS - 1)));
do
    AGENT=${AGENT_LIST[i]}
    ssh -i IOENTest.txt ubuntu@$AGENT "rm -rf ~/testresults"
done
ssh -i ~/IOENTest.txt ubuntu@$MARKET_AGENT "rm -rf ~/testresults"
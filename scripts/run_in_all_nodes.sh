#!/bin/bash

IFS=$'\n' read -d '' -r -a AGENT_LIST < ~/scripts/agent_list.txt

for i in ${AGENT_LIST[@]};
do
    ssh -v -i IOENTest.txt ubuntu@$i $1
done

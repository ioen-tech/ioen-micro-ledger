#!/bin/bash

IFS=$'\n' read -d '' -r -a AGENT_LIST < ~/scripts/agent_list.txt

for i in ${AGENT_LIST[@]};
do
    scp -oStrictHostKeyChecking=no -v -r -i IOENTest.txt ~/scripts ubuntu@$i:~/
done

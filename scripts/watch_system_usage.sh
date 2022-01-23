#!/bin/bash

while true
do 
    CPU=$((100-$(vmstat 1 2|tail -1|awk '{print $15}')))
    FREE_DATA=`free -m | grep Mem` 
    CURRENT=`echo $FREE_DATA | cut -f3 -d' '`
    TOTAL=`echo $FREE_DATA | cut -f2 -d' '`
    RAM=$(free -m | awk 'NR==2{printf "%.2f%%\n", $3*100/$2 }')
    HDD=$(df -lh | awk '{if ($6 == "/") { print $5 }}' | head -1 | cut -d'%' -f1)
    TIMESTAMP=$( date +%T )

    echo TIME $TIMESTAMP: CPU $CPU %, RAM $RAM %, HDD $HDD %
done
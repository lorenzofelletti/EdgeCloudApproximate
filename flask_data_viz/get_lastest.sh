#!/bin/bash
LAST=$(ls -td /results/* | head -1 | rev | cut -d_ -f1 | rev)
cat /results/*$LAST/p*
echo "geohash,avg_speed" > latest.csv
cat /results/*$LAST/p* | grep -v geohash >> latest.csv

#!/bin/bash
base_dir=/vol/avg_speed
flask_dir=/flask_data_viz
latest=$(ls -t $base_dir | grep ^p | head -n 1)
cat $base_dir/$latest > $flask_dir/latest.csv
date=$(date -r $base_dir/$latest +"%Y-%m-%d %H:%M:%S")
echo $date > $flask_dir/latest_meta.txt

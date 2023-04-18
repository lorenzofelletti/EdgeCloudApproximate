#!/bin/bash
sleep 15

# execute flask app in separate terminal
bash -c "cd /flask_data_viz && python3 main.py" &

# Set startup script
spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py &

while true; do
    sleep 1000
    # execute get_latest.sh without exiting on failure
    bash -c "/get_latest.sh" || true
done

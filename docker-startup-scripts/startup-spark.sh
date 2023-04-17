#!/bin/bash
sleep 25

# execute flask app in separate terminal
bash -c "cd /flask_data_viz && python3 main.py" &

# Set startup script
spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py

#!/bin/bash
sleep 25

# Set startup script
spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py

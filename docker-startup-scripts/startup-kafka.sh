#!/bin/bash
./kafka-producer-rs/kafka-producer-rs create_topic
./kafka-edge-rs topic create out --for-nbw-strat

./kafka-producer-rs/kafka-producer-rs &
./kafka-edge-rs -s 0.8

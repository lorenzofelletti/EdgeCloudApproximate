import geohash2 as geohash
import pandas as pd
from kafka import KafkaConsumer, TopicPartition
from datetime import datetime, timedelta
import random

# Define Kafka topic and bootstrap servers
topic = 'spatial1'
bootstrap_servers = ['localhost:9092']

# Define Kafka consumer
consumer = KafkaConsumer(bootstrap_servers=bootstrap_servers)

# Define partition to use
partition_to_use = 0

# Assign partition to consumer
consumer.assign([TopicPartition(topic, partition_to_use)])
consumer.seek_to_beginning()

# Define window length in seconds
window_length = 300

# Define sampling percentage
sampling_frac = 0.5

# Define function for stratified sampling
def stratified_sampling(df, frac=sampling_frac):
    # Group data by geohash
    grouped = df.groupby('geohash')
    # Sample frac of data for each geohash
    sampled = grouped.apply(lambda x: x.sample(frac=frac))
    return sampled

# Define function to extract geohash from latitude and longitude


def get_geohash(lat, long):
    return geohash.encode(lat, long)


# Define empty dataframe to hold data
data = pd.DataFrame(columns=['id', 'lat', 'long', 'speed'])

# Define start time for window
window_start = datetime.now()

# Loop through Kafka messages for this partition
for msg in consumer:
    # Parse message data
    msg_data = msg.value.decode('utf-8').split(',')
    id = msg_data[0]
    lat = float(msg_data[1])
    long = float(msg_data[2])
    speed = float(msg_data[3])
    # Add data to dataframe
    data = data.append({'id': id, 'lat': lat, 'long': long,
                       'speed': speed}, ignore_index=True)
    # Check if window has elapsed
    if datetime.now() - window_start > timedelta(seconds=window_length):
        # Get geohash for each data point
        data['geohash'] = data.apply(
            lambda row: get_geohash(row['lat'], row['long']), axis=1)
        # Perform stratified sampling
        sampled_data = stratified_sampling(data)
        # Output sampled data
        print(sampled_data)
        # Reset dataframe for next window
        data = pd.DataFrame(columns=['id', 'lat', 'long', 'speed'])
        # Reset window start time
        window_start = datetime.now()

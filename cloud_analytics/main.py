# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py
import time
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, TimestampType, TimestampNTZType

KAFKA_BOOTSTRAP_SERVERS = "kafka:9092"
KAFKA_TOPIC = "dataout.*"
FILE_PATH = "/data/china/neighborhood"
OUTPUT_PATH = "/results/"

SCHEMA = StructType([
    StructField("id", StringType(), False),
    StructField("lat", DoubleType(), False),
    StructField("lon", DoubleType(), False),
    StructField("time", StringType(), False),
    StructField("speed", DoubleType(), False),
    StructField("geohash", StringType(), False),
    StructField("neighborhood", StringType(), False),
])

spark: SparkSession = SparkSession.builder.appName(
    "write_traffic_sensor_topic").getOrCreate()

df_stream = spark\
    .readStream.format("kafka")\
    .option("kafka.bootstrap.servers", KAFKA_BOOTSTRAP_SERVERS)\
    .option("subscribePattern", KAFKA_TOPIC)\
    .option("startingOffsets", "earliest")\
    .load()

# deserialize the data from kafka
df_stream = df_stream.selectExpr("CAST(value AS STRING)")
# json deserialie using schema
df_stream = df_stream.select(F.from_json(
    F.col("value"), SCHEMA).alias("data")).select("data.*")

# 2014-10-22T19:40:05.000Z
df_stream = df_stream.withColumn("timestamp", F.unix_timestamp(
    F.col("time"), "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'").cast(TimestampType())).withWatermark("timestamp", "1 minutes")

# group by geohash and calculate the average speed of each geohash
gh_stream = df_stream.groupBy(
    F.col("geohash"), "timestamp").agg(F.avg("speed").alias("avg_speed"))
# write the results to a file
gh_stream.writeStream\
    .queryName("geohashAvg")\
    .format("csv")\
    .trigger(processingTime='1 minutes')\
    .option("checkpointLocation", "checkpoint/")\
    .option("path", OUTPUT_PATH)\
    .outputMode("append")\
    .start()\
    .awaitTermination()

# print the results every 30 s
while True:
    x = spark.sql("SELECT * FROM geohashAvg")
    x.head(10)
    x.show()
    time.sleep(30)

#nb_stream = df_stream.groupBy(
#    "neighborhood").agg(F.avg("speed").alias("avg_speed"))
## write the results
#nb_stream.writeStream\
#    .queryName("neighborhoodAvg")\
#    .format("memory")\
#    .trigger(processingTime='1 minutes')\
#    .option("checkpointLocation", "checkpoint/")\
#    .outputMode("append")\
#    .start()\
#    .awaitTermination()

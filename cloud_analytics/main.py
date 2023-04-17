# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py
import time
from datetime import datetime
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, TimestampType, TimestampNTZType

KAFKA_BOOTSTRAP_SERVERS = "kafka:9092"
KAFKA_TOPIC = ["dataout_1", "dataout_2", "dataout_3",
               "dataout_4", "dataout_5", "dataout_6"]

OUTPUT_PATH = "/results/"


def prepare_query(spark: SparkSession, topic_name: str):
    df_stream = spark\
        .readStream.format("kafka")\
        .option("kafka.bootstrap.servers", KAFKA_BOOTSTRAP_SERVERS)\
        .option("subscribe", topic_name)\
        .option("startingOffsets", "earliest")\
        .load()

    # deserialize the data from kafka
    df_stream = df_stream.selectExpr("CAST(value AS STRING)")
    # json deserialie using schema
    df_stream = df_stream.select(F.from_json(
        F.col("value"), SCHEMA).alias("data")).select("data.*")

    # creates a write stream with the query name
    df_stream.writeStream.queryName(
        "query_" + topic_name).format("memory").outputMode("complete").start()


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

# set log level to WARN
spark.sparkContext.setLogLevel("WARN")

# create a query for each topic
for topic in KAFKA_TOPIC:
    prepare_query(spark, topic)

while True:
    time.sleep(60 * 2)
    
    # for each table compute the average speed by geohash and union the results
    tot = None
    for topic in KAFKA_TOPIC:
        df = spark.sql(
            f"select geohash, avg(speed) as avg_speed from query_{topic} group by geohash")
        if tot is None:
            tot = df
        else:
            tot = tot.union(df)
    # save the results in a csv file
    filename = "avg_speed_" + time.time_ns()

# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py
import time
from datetime import datetime
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, TimestampType, TimestampNTZType


SCHEMA = StructType([
    StructField("id", StringType(), False),
    StructField("lat", DoubleType(), False),
    StructField("lon", DoubleType(), False),
    StructField("time", StringType(), False),
    StructField("speed", DoubleType(), False),
    StructField("geohash", StringType(), False),
    StructField("neighborhood", StringType(), False),
])

KAFKA_BOOTSTRAP_SERVERS = "kafka:9092"

OUTPUT_PATH = "/results/"

TOPICS = ["dataout_1", "dataout_2", "dataout_3",
          "dataout_4", "dataout_5", "dataout_6"]

spark: SparkSession = SparkSession.builder.appName(
    "write_traffic_sensor_topic").getOrCreate()

# set log level to WARN
spark.sparkContext.setLogLevel("WARN")

df_stream = spark\
    .readStream.format("kafka")\
    .option("kafka.bootstrap.servers", KAFKA_BOOTSTRAP_SERVERS)\
    .option("subscribe", "dataout_*")\
    .option("startingOffsets", "earliest")\
    .load()
# deserialize the data from kafka
df_stream = df_stream.selectExpr("CAST(value AS STRING)")
# json deserialie using schema
df_stream = df_stream.select(F.from_json(
    F.col("value"), SCHEMA).alias("data")).select("data.*")

df_stream = df_stream.withColumn("curr_timestamp", F.current_timestamp())

# keep only data from the last 10 minutes
df_stream = df_stream.withWatermark("curr_timestamp", "10 minutes")

# writeStream in append mode data in a rolling window of 10 minutes
df_stream.writeStream.queryName("lastTen").format("memory").outputMode("append")\
    .trigger(processingTime="30 seconds")\
    .start()

while True:
    time.sleep(30)
    df = spark.sql(
            f"select geohash, avg(speed) as avg_speed from query_{topic}group by geohash")
    df.show()
    print("Number of rows: ", df.count())
    filename = "avg_speed_" + \
            datetime.now().strftime("%Y%m%d_%H%M")
    df.write.format("csv").option("header", "true").mode(
            "overwrite").save(OUTPUT_PATH + filename)

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


df_stream = spark\
    .readStream.format("kafka")\
    .option("kafka.bootstrap.servers", KAFKA_BOOTSTRAP_SERVERS)\
    .option("subscribe", "dataout_1,dataout_2,dataout_3,dataout_4,dataout_5,dataout_6")\
    .option("startingOffsets", "earliest")\
    .load()
# deserialize the data from kafka
df_stream = df_stream.selectExpr("CAST(value AS STRING)")
# json deserialie using schema
df_stream = df_stream.select(F.from_json(
    F.col("value"), SCHEMA).alias("data")).select("data.*")

df_stream = df_stream.withColumn("timestamp", F.current_timestamp())
#df_stream = df_stream.withWatermark("timestamp", "1 minutes")

df_stream = df_stream.groupBy(F.window("timestamp", "1 minutes"), F.col("geohash"))\
    .agg(F.avg("speed").alias("avg_speed"))
# creates a write stream with the query name
df_stream.writeStream\
    .queryName("query").format("memory").trigger(processingTime="30 seconds")\
        .outputMode("complete").start()

time.sleep(70)
while True:
    df = spark.sql(f"select * from query")
    df.show()
    # save the results in a csv file
    filename = "avg_speed_" + str(time.time_ns())
    df.write.csv(OUTPUT_PATH + filename)
    
    time.sleep(60)

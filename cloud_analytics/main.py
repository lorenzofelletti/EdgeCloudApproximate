# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py
import time
from datetime import datetime
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, TimestampType, TimestampNTZType

KAFKA_BOOTSTRAP_SERVERS = "kafka:9092"
KAFKA_TOPICS = "dataout_0,dataout_1,dataout_2,dataout_3,dataout_4,dataout_5,dataout_6,dataout_e"

OUTPUT_PATH = "/vol/"

SCHEMA = StructType([
    StructField("id", StringType(), False),
    StructField("lat", DoubleType(), False),
    StructField("lon", DoubleType(), False),
    StructField("time", TimestampType(), False),
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
    .option("subscribe", KAFKA_TOPICS)\
    .option("startingOffsets", "earliest")\
    .load()
# deserialize the data from kafka
df_stream = df_stream.selectExpr("CAST(value AS STRING)")
# json deserialie using schema
df_stream = df_stream.select(F.from_json(
    F.col("value"), SCHEMA).alias("data")).select("data.*")

df_stream = df_stream.groupBy(F.window("time", "30 minutes"), F.col("geohash"))\
    .agg(F.avg("speed").alias("avg_speed"))
# creates a write stream with the query name
df_stream.writeStream\
    .queryName("query").format("memory").trigger(processingTime="30 seconds")\
    .outputMode("complete").start()

time.sleep(35)
while True:
    df = spark.sql(f"select * from query")
    if df is not None:
        df.show()
        df = df.orderBy(F.col("window.start").desc())
        #stringify the time
        df = df.withColumn("window", F.col("window.end").cast(StringType()))
        df = df.withColumnRenamed("window", "time") # rename the column to time
        # save the results in a csv file
        filename = "avg_speed"
        df.repartition(1).write\
            .mode("overwrite")\
            .option("header",True)\
            .csv(OUTPUT_PATH + filename)
    
    time.sleep(60)

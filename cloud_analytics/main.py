# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py
import datetime
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, IntegerType, TimestampType, LongType, DoubleType

KAFKA_BOOTSTRAP_SERVERS = "kafka:9092"
KAFKA_TOPIC = "datain"
FILE_PATH = "/data/china/neighborhood"
OUTPUT_PATH = "/results/"

SCHEMA = StructType([
    StructField("id", StringType()),
    StructField("lat", DoubleType()),
    StructField("lon", DoubleType()),
    StructField("time", StringType()),
    StructField("speed", DoubleType()),
])

spark: SparkSession = SparkSession.builder.appName(
    "write_traffic_sensor_topic").getOrCreate()

df_traffic_stream = spark\
    .readStream.format("kafka")\
    .option("kafka.bootstrap.servers", KAFKA_BOOTSTRAP_SERVERS)\
    .option("subscribe", KAFKA_TOPIC)\
    .option("startingOffsets", "earliest")\
    .load()

df_traffic_stream.select(
    # Convert the value to a string
    F.from_json(
        F.decode(F.col("value"), "iso-8859-1"),
        SCHEMA
    ).alias("value")
)\
    .select("value.*")

# calculate the geohash of each point using geohash2 library
df_traffic_stream = df_traffic_stream.withColumn(
    "geohash", F.expr("geohash2.encode(lat, lon, 6)"))

# group by geohash and calculate the average speed of each geohash
df_traffic_stream = df_traffic_stream.groupBy(
    "geohash").agg(F.avg("speed").alias("avg_speed"))

# write the result to a file
df_traffic_stream.writeStream\
    .outputMode("complete")\
    .format("csv")\
    .option("path", OUTPUT_PATH + datetime.now().strftime("%Y-%m-%d-%H-%M-%S"))\


# .writeStream\
# .outputMode("append")\
# .format("memory")\
# .start()\
# .awaitTermination()

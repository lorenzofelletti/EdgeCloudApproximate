# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/main.py
from datetime import datetime
import time
import geohash2
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, IntegerType, TimestampType, LongType, DoubleType

KAFKA_BOOTSTRAP_SERVERS = "kafka:9092"
KAFKA_TOPIC = "dataout*"
FILE_PATH = "/data/china/neighborhood"
OUTPUT_PATH = "/results/"

SCHEMA = StructType([
    StructField("id", StringType(), False),
    StructField("lat", DoubleType(), False),
    StructField("lon", DoubleType(), False),
    StructField("time", StringType(), False),
    StructField("speed", DoubleType(), False),
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

# calculate the geohash of each point using geohash2 library
udf1 = F.udf(lambda x, y: geohash2.encode(x, y, precision=6))
df_stream = df_stream.withColumn(
    "geohash", udf1('lat', 'lon'))

# group by geohash and calculate the average speed of each geohash
df_stream = df_stream.groupBy(
    "geohash").agg(F.avg("speed").alias("avg_speed"))

# write the result to a file
df_stream.writeStream\
    .queryName("queryTable")\
    .format("memory")\
    .outputMode("complete")\
    .start()

time.sleep(5)

pop = spark.sql("select * from queryTable")
time.sleep(5)
pop.createOrReplaceTempView("updates")
time.sleep(5)
pop.show()
spark.sql("select * from updates").show()
time.sleep(5)
pop.show()
# sleep for 30 seconds to wait for the query to start
time.sleep(15)
pop.show()
time.sleep(15)
pop.write.format("csv").option("header", "true").mode(
    "overwrite").save(OUTPUT_PATH)

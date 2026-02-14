# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/pm25_agg.py csv-data-agg/combined.csv
import sys
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, DoubleType

def main():
    # Check if input file is provided
    if len(sys.argv) != 2:
        print("Usage: spark-submit cloud_analytics/pm25_agg.py <input_csv_file>")
        sys.exit(-1)

    input_path = sys.argv[1]

    # Initialize Spark Session
    spark = SparkSession.builder \
        .appName("PM25_Neighborhood_Aggregator") \
        .getOrCreate()

    # Set log level to WARN to reduce verbosity
    spark.sparkContext.setLogLevel("WARN")

    # Define the schema corresponding to the CSV header provided
    schema = StructType([
        StructField("City", StringType(), True),
        StructField("DeviceId", StringType(), True),
        StructField("LocationName", StringType(), True),
        StructField("Latitude", DoubleType(), True),
        StructField("Longitude", DoubleType(), True),
        StructField("ReadingDateTimeUTC", StringType(), True),
        StructField("PM25", DoubleType(), True),
        StructField("CalibratedPM25", DoubleType(), True),
        StructField("CalibratedO3", DoubleType(), True),
        StructField("CalibratedNO2", DoubleType(), True),
        StructField("CO", DoubleType(), True),
        StructField("Temperature", DoubleType(), True),
        StructField("Humidity", DoubleType(), True),
        StructField("BatteryLevel", DoubleType(), True),
        StructField("PercentBattery", DoubleType(), True),
        StructField("CellSignal", DoubleType(), True),
        StructField("geohash", StringType(), True),
        StructField("neighborhood", StringType(), True)
    ])

    # Read the CSV file
    df = spark.read \
        .option("header", "true") \
        .schema(schema) \
        .csv(input_path)

    # Perform aggregation
    # Group by 'neighborhood' and calculate average of 'PM25'
    result_df = df.groupBy("neighborhood") \
        .agg(F.avg("PM25").alias("avg_PM25")) \
        .orderBy("avg_PM25", ascending=False)

    # Show the results
    result_df.show(truncate=False)

if __name__ == "__main__":
    main()

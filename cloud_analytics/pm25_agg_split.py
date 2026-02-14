# spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.3.0 cloud_analytics/pm25_agg_split.py csv-data
import sys
import glob
import os
from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from pyspark.sql.types import StructType, StructField, StringType, DoubleType

def main():
    # Check if input directory is provided
    if len(sys.argv) != 2:
        print("Usage: spark-submit cloud_analytics/pm25_agg_split.py <input_directory>")
        sys.exit(-1)

    input_dir = sys.argv[1]

    # Initialize Spark Session
    spark = SparkSession.builder \
        .appName("PM25_Neighborhood_Aggregator_Split") \
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

    # Get list of CSV files in the directory
    # We use glob to list files and filter out the one ending in _e.csv
    path_pattern = os.path.join(input_dir, "*.csv")
    all_files = glob.glob(path_pattern)
    
    # Filter out files ending with _e.csv (error/unaggregated data)
    input_files = [f for f in all_files if not f.endswith("_e.csv")]

    if not input_files:
        print(f"No valid CSV files found in {input_dir} (excluding *_e.csv)")
        sys.exit(0)

    # Read the filtered CSV files
    df = spark.read \
        .option("header", "true") \
        .schema(schema) \
        .csv(input_files)

    # Perform aggregation
    result_df = df.groupBy("neighborhood") \
        .agg(F.avg("PM25").alias("avg_PM25")) \
        .orderBy("avg_PM25", ascending=False)

    # Show the results
    result_df.show(truncate=False)

if __name__ == "__main__":
    main()

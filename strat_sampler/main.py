import argparse
import os
import sys
import time
import pandas as pd


def write(entry, file_desc, ofs=None):
    """Write entry to file or stdout"""
    if ofs is not None:
        entry = entry.replace(",", ofs)
    if file_desc == sys.stdout:
        print(entry)
    else:
        file_desc.write(entry)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Sample a csv file by geohash and output to stdout or file (default: stdout)"
    )
    parser.add_argument(
        "input", type=str, help="file to sample", metavar="<input_file>"
    )
    parser.add_argument(
        "--IFS",
        "-i",
        type=str,
        default=",",
        help="input field separator. (default: ,)",
        metavar="<sep>",
    )
    parser.add_argument(
        "--sample-percentage",
        "-s",
        type=float,
        default=0.5,
        help="percentage of entries to sample (default: 0.5)",
        metavar="<perc>",
    )
    parser.add_argument(
        "--output-file",
        "-o",
        type=str,
        default="stdout",
        help="file to write to (default: stdout)",
        metavar="<output_file>",
    )
    parser.add_argument(
        "--sleep",
        "-t",
        type=float,
        default=0.0,
        help="sleep for x seconds between each entry, 0 means no sleep (default: 0)",
        metavar="<secs>",
    )
    parser.add_argument(
        "--OFS",
        type=str,
        default=None,
        help="output field separator (default: is IFS)",
        metavar="<sep>",
    )

    # parse arguments
    args = parser.parse_args()

    # check if input file exists
    if not os.path.exists(args.input):
        print("Input file does not exist")
        sys.exit(1)

    # parse output file
    output = args.output_file if args.output_file != "stdout" else sys.stdout

    # parse output field separator
    if args.OFS is None:
        args.OFS = args.IFS

    # read csv file
    df = pd.read_csv(args.input, sep=args.IFS)

    unique_geohashes = df["geohash"].unique()

    # count entries per geohash
    count_per_geohash = df.groupby("geohash").count()
    # df['count'] = df.groupby('geohash')['geohash'].transform('count')

    # randomly drop "sample_percentage" of entries per geohash
    df = df.groupby("geohash").apply(lambda x: x.sample(frac=args.sample_percentage))
    # return to original index order
    df = df.droplevel(0).reset_index(drop=True)
    # sort by id
    df = df.sort_values(by=["id"])

    # write to file or stdout one entry at a time (if sleep is specified)
    if args.sleep == 0:
        # write directly to file or stdout
        df.to_csv(output, index=False, header=False, sep=args.OFS)
    else:
        # write to file or stdout one entry at a time
        # open file if output is not stdout
        if output != sys.stdout:
            output = open(args.output_file, "a+")

        # write one entry at a time
        for i in range(len(df)):
            df.loc[[i]].to_csv(
                output, index=False, header=False, mode="a", sep=args.OFS
            )

            time.sleep(args.sleep)

        # for entry in df:
        #    write(entry, output, ofs=args.OFS)

        #    time.sleep(args.sleep)

        # close file if output is not stdout
        if output != sys.stdout:
            output.close()

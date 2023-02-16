import argparse


def build_parser():
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

    return parser

from flask import Flask, render_template
import os

from geomap import process_data

app = Flask(__name__)

RESULTS_DIR = '/results'

def get_str_after_last_underscore(s: str) -> str:
    return s.split('_')[-1]

def get_results_dirs_ending_with(s: str) -> list[str]:
    return [d for d in os.listdir(RESULTS_DIR) if d.endswith(s)]

def get_latest_results_files() -> list[str]:
    '''Returns the list of files resulting from the latest run.'''
    # check in the /results (absolute path) folder for the latest directory name
    latest_dir = max(os.listdir(RESULTS_DIR), key=os.path.getctime)
    # decompose the name to get everything after the last underscore
    latest_run = get_str_after_last_underscore(latest_dir)
    # get all the directories in /results ending with the latest run number
    latest_dirs = get_results_dirs_ending_with(latest_run)
    # get all the files in the latest directories
    latest_files = [os.path.join(d, f) for d in latest_dirs for f in os.listdir(d)]
    return latest_files


@app.route('/')
def home():
    latest_files = get_latest_results_files()

    # join the contents of all the files into a single one
    #latest_file_name = os.path.join(RESULTS_DIR, 'latest.txt')
    latest_file_name = 'latest.txt'
    #removes all the lines that start with the word "geohash"
    with open(latest_file_name, 'w') as outfile:
        outfile.write('geohash,avg_speed')
        for fname in latest_files:
            with open(fname) as infile:
                for line in infile:
                    # remove the header line
                    if not line.startswith('geohash'):
                        outfile.write(line)
    # the csv file contains the average speed for each geohash
    # create a map visualizing the data
    process_data(latest_file_name)

    return render_template('map.html')

@app.route('/test')
def test():
    process_data('test.csv')
    return render_template('map.html')


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)

from flask import Flask, render_template, request
import os

import pandas as pd

from geomap import process_data

app = Flask(__name__)

def get_latest():
    '''Run the get_latest.sh script to get the latest data'''
    os.system('./get_latest.sh')


@app.route('/', methods=['GET'])
def home():
    get_latest()

    update_time = None
    # get time from latest_meta.txt
    try:
        with open('latest_meta.txt', 'r') as f:
            update_time = f.read().strip()
    except FileNotFoundError:
        pass

    data = None
    try:
        data = pd.read_csv('latest.csv')
    except Exception:
        pass
    
    if data is None or data.empty:
        return render_template('waiting.html')

    # get time if provided
    time = request.args.get('time')
    if time is not None:
        time = time.replace('_', ' ')
    else:
        time = data['time'][0]
    
    process_data(data, time)

    available_times = data['time'].unique()
    # convert whitespaces to underscores
    available_times = [t.replace(' ', '_') for t in available_times]

    return render_template('index.html', time=time, times=available_times, latest_update=update_time)


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)

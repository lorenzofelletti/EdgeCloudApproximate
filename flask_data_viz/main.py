from flask import Flask, render_template
import os

from geomap import process_data

app = Flask(__name__)


def get_latest():
    '''Execute shell script get_latest.sh'''
    os.system('./get_latest.sh')


@app.route('/')
def home():
    # refresh the data
    get_latest()

    process_data('latest.csv')

    return render_template('map.html')

@app.route('/test')
def test():
    process_data('test.csv')
    return render_template('map.html')


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)

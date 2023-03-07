# Cloud Analytics Notebook
This notebook reads from the Kafka topic (spatial1) and calculates the average speed in each geohash.

## How to start it
 - Run `conda activate ENV_NAME` (e.g. `base`)
 - Start livy server `~/apache-livy-0.7.1-incubating-bin/bin/livy-server start` (or whatever is your path to Livy)
 - Run `jupyter notebook`
 - Open notebook and run code.

## How to open it in VSCode
> First, make sure the necessary Jupyter extensions are installed.
 - Open the notebook file
 - In the top-right corner click to select a kernel to use for the notebook
 - Choose to use the Jupyter URL
 - Copy/paste the URL of the current Juptyer notebook.
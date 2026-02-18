#!/bin/bash

export PYTHONPATH="${SPARK_HOME}/python/lib/py4j-0.10.9.5-src.zip:${PYTHONPATH}"

jupyter notebook --ip 0.0.0.0 --no-browser --allow-root

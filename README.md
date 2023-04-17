# EdgeApproximate

## Environment Setup
- Install Docker Desktop
- Run `docker-compose up -d` in the root directory of the project
- Enjoy!

## Containers Configuration
- There are 5 containers in total
  - One Spark Master
  - One Spark Worker
  - One Zookeeper node
  - Two Kafka nodes
    - Kafka (Main Kafka broker)
    - Edge1 (Edge node).

On the Spark master node, runs a pyspark script that continuously reads from the DATAOUT topic(s) of the Kafka broker and writes the processed data to a local directory. Also on the Spark master node, runs a Flask server that serves the processed data to the frontend. The Flask server is accessible at `localhost:5000`.

On the Zookeeper node, runs a Zookeeper server (required by Kafka).

On the Kafka node, two programs runs:
- `kafka-producer-rs` -- A Rust program that reads from a local csv the data to distribute to the Kafka nodes via a DATAIN topic; simulating the data arriving to the edge nodes.
- `kafka-edge-rs` -- A Rust program that reads from the DATAIN topic and writes to one or more DATAOUT topics (depending on the configuration); simulating being an edge node.

On the Edge1 node, runs the same `kafka-edge-rs` program as the Kafka node, but withouth the `kafka-producer-rs` program (required only on one node).

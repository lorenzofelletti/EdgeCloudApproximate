edge_images = {
  "edge0" = {
    dockerfile = "Dockerfile_kafka"
  }
  "edge1" = {
    dockerfile = "Dockerfile_edge"
  }
}

spark_images = {
  master = {
    name       = "spark"
    dockerfile = "Dockerfile_spark"
  }
  worker = {
    name = "docker.io/bitnami/spark:3.3"
  }
}

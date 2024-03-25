edge_images = {
  "producer" = {
    dockerfile = "Dockerfile_k8s_producer"
  }
  "edge" = {
    dockerfile = "Dockerfile_k8s_edge"
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

cluster_name = "edge-cloud-approximate"

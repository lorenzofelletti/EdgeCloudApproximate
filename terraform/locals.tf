locals {
  images_to_load = concat(
    [for image in docker_image.edge : image.name],
    [docker_image.spark_master.name, docker_image.spark_worker.name],
  )
}

resource "docker_image" "edge" {
  for_each = var.edge_images

  name = each.key

  build {
    context    = "${path.module}/${each.value.context}"
    dockerfile = each.value.dockerfile
  }
}

resource "docker_image" "spark_master" {
  name = var.spark_images.master.name

  build {
    context    = "${path.module}/${var.spark_images.master.context}"
    dockerfile = var.spark_images.master.dockerfile
  }
}

resource "docker_image" "spark_worker" {
  name = var.spark_images.worker.name

  dynamic "build" {
    for_each = var.spark_images.worker.dockerfile != null ? toset(1) : {}

    content {
      context    = "${path.module}/${var.spark_images.worker.context}"
      dockerfile = var.spark_images.worker.dockerfile
    }
  }
}

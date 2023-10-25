variable "cluster_name" {
  description = "The name of the minikube cluster"
  type        = string
  default     = "edge-cloud-approximate"
}

variable "edge_images" {
  description = <<-EOT
  Edge nodes images to build.
  The key is the name of the image, the value is a map with the following keys:
  - context: the path to the directory containing the Dockerfile, relative to `"$${path.module}/"` (default: "..")
  - dockerfile: the name of the Dockerfile (relative to `context`)
  EOT
  type = map(object({
    context    = optional(string, "..")
    dockerfile = string
  }))
}

variable "spark_images" {
  description = ""
  type = object({
    master = object({
      name       = optional(string, "spark")
      context    = optional(string, "..")
      dockerfile = string
    })
    worker = object({
      name       = optional(string, "spark-worker")
      context    = optional(string, null)
      dockerfile = optional(string, null)
    })
  })

  validation {
    condition     = (var.spark_images.worker.dockerfile != null && var.spark_images.worker.context != null) || can(regex(".+:.+", var.spark_images.worker.name))
    error_message = "Either `spark_images.worker.dockerfile` and `spark_images.worker.context` must be set, or `spark_images.worker.name` must be in the form `image:tag`"
  }
}

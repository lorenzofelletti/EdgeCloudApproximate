variable "image" {
  description = "The Spark image to use, in the format `repository:tag`."
  type        = string
  default     = "docker.io/bitnami/spark:3.4"

  validation {
    condition     = can(regex("^[a-z0-9-._/]+:[a-z0-9-._]+$", var.image))
    error_message = "Image must match regex `^[a-z0-9-._/]+:[a-z0-9-._]+$`."
  }

  validation {
    condition     = can(regex(".*bitnami/spark:.*", var.image))
    error_message = "Image must be a Bitnami Spark image."
  }
}

variable "name" {
  description = "Name of the Spark cluster. This will be used as the namespace, deployments, etc."
  type        = string
  default     = "spark"
  nullable    = false

  validation {
    condition     = can(regex("^[a-z][a-z.-]{,252}+$", var.name))
    error_message = "Name must match regex `^[a-z][a-z.-]{,252}+$`."
  }
}

variable "service_account_name" {
  description = "Name of the service account"
  type        = string
  default     = null

  validation {
    condition     = can(regex("^[a-z][a-z.-]{,252}+$", var.service_account_name))
    error_message = "service_account_name must match regex `^[a-z][a-z.-]{,252}+$`."
  }
}

variable "app_label_key" {
  description = <<-EOT
  The key name for the app label. For example, `app.kubernetes.io/name`.
  The value will be set equal to the `name` variable."
  EOT
  type        = string
  default     = "app.kubernetes.io/name"
  nullable    = false
}

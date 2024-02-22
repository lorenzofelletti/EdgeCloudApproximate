variable "image" {
  description = "The Kafka image to use, in the format `repository:tag`."
  type        = string
  default     = "docker.io/bitnami/kafka:3.4"

  validation {
    condition     = can(regex("^[a-z0-9-._/]+:[a-z0-9-._]+$", var.image))
    error_message = "Image must match regex `^[a-z0-9-._/]+:[a-z0-9-._]+$`."
  }

  validation {
    condition     = can(regex(".*bitnami/kafka:.*", var.image))
    error_message = "Image must be a Bitnami Kafka image."
  }
}

variable "name" {
  description = "Name of the Kafka cluster. This will be used as the namespace, service, and stateful set name."
  type        = string
  default     = "kafka"
  nullable    = false

  validation {
    condition     = can(regex("^[a-z][a-z.-]{,252}+$", var.name))
    error_message = "Name must match regex `^[a-z][a-z.-]{,252}+$`."
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

variable "service_account_name" {
  description = "Name of the service account"
  type        = string
  default     = null

  validation {
    condition     = can(regex("^[a-z][a-z.-]{,252}+$", var.service_account_name))
    error_message = "service_account_name must match regex `^[a-z][a-z.-]{,252}+$`."
  }

}

variable "service_name_template" {
  description = <<-EOT
    Template for the service name. The default value is "{{name}}-headless". The following placeholders are available:
    - {{name}}: The name of the Kafka cluster (got from the `name` variable)
  EOT
  type        = string
  default     = "{{name}}-headless"
  nullable    = false
}

variable "replicas" {
  description = "Number of Kafka brokers to deploy"
  type        = number
  default     = 3
  nullable    = false
}

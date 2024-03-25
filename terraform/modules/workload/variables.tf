variable "edge_workload_namespace" {
  description = "Name to give to the namespace of the edge workload"
  type        = string
  default     = "edge-cloud-approximate"
}

variable "edge_workload_selector" {
  description = "Selector to use for the edge workload"
  type = object({
    key   = optional(string, "app.kubernetes.io/name")
    value = optional(string, "edge-cloud-approximate")
  })
  default = {
    key   = "app.kubernetes.io/name"
    value = "edge-cloud-approximate"
  }
}

variable "edge_workload_producer_selector" {
  description = "Selector to use for the edge workload producer"
  type = object({
    key   = optional(string, "app.kubernetes.io/name")
    value = optional(string, "edge-cloud-approximate-producer")
  })
  default = {
    key   = "app.kubernetes.io/name"
    value = "edge-cloud-approximate-producer"
  }
}

variable "edge_workload_image" {
  description = "Image to use for the edge workload"
  type        = string
}

variable "edge_producer_image" {
  description = "Image to use for the edge workload producer"
  type        = string
}

locals {
  labels          = merge(var.edge_workload_selector)
  producer_labels = merge(var.edge_workload_producer_selector)
}

locals {
  annotations = {
    "${var.app_label_key}" = var.name
  }

  labels = {
    "${var.app_label_key}" = var.name
  }

  master_labels = merge(local.labels, {
    "component" = "spark-master"
  })

  client_labels = merge(local.labels, {
    "component" = "spark-client"
  })

  service_account_name = coalesce(var.service_account_name, var.name)
}

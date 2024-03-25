module "kafka" {
  source   = "../kafka"
  replicas = 2
}

resource "kubernetes_namespace_v1" "edge-cloud-approximate" {
  metadata {
    name = var.edge_workload_namespace
  }
}

resource "kubernetes_service_account_v1" "edge-cloud-approximate" {
  metadata {
    name      = var.edge_workload_namespace
    namespace = kubernetes_namespace_v1.edge-cloud-approximate.metadata[0].name
  }
}

resource "kubernetes_service_v1" "edge-cloud-approximate" {
  metadata {
    name      = "${var.edge_workload_namespace}-headless"
    namespace = kubernetes_namespace_v1.edge-cloud-approximate.metadata[0].name
    labels    = local.labels
  }
  spec {
    cluster_ip = "None"
    selector   = var.edge_workload_selector
  }
}

resource "kubernetes_stateful_set_v1" "edge-cloud-approximate" {
  metadata {
    name      = var.edge_workload_namespace
    namespace = kubernetes_namespace_v1.edge-cloud-approximate.metadata[0].name
    labels    = local.labels
  }
  spec {
    replicas     = 2
    service_name = kubernetes_service_v1.edge-cloud-approximate.metadata[0].name
    selector {
      match_labels = local.labels
    }
    template {
      metadata {
        labels = local.labels
      }
      spec {
        service_account_name = kubernetes_service_account_v1.edge-cloud-approximate.metadata[0].name
        init_container {
          name  = "setup"
          image = "busybox"
          command = [
            "sh",
            "-c",
            <<-EOT
            let "PARTITION = $${HOSTNAME##*-} + 1"
            sed -E -i "s/(partitions_to_consume[[:space:]]*=[[:space:]]*\[)(.*)(\])/\1 \"$PARTITION\" \3/g" /home/nonroot/kafka_edge_config.toml
            echo /home/nonroot/kafka_edge_config.toml
            EOT
          ]

          resources {
            limits = {
              cpu    = "100m"
              memory = "64Mi"
            }
          }
        }

        container {
          name    = var.edge_workload_namespace
          image   = var.edge_workload_image
          command = ["/home/nonroot/kafka-edge-rs", "-s=0.8"]

          resources {
            limits = {
              cpu               = "200m"
              ephemeral-storage = "256Mi"
              memory            = "256Mi"
            }
            requests = {
              cpu    = "100m"
              memory = "128Mi"
            }
          }
        }
      }
    }
  }

}

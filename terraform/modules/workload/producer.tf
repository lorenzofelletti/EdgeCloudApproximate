resource "kubernetes_deployment_v1" "producer" {
  metadata {
    name      = "${var.edge_workload_namespace}-producer"
    labels    = local.labels
    namespace = kubernetes_namespace_v1.edge-cloud-approximate.metadata[0].name
  }
  spec {
    selector {
      match_labels = local.producer_labels

    }
    template {
      metadata {
        labels = local.producer_labels
      }
      spec {
        container {
          name    = "${var.edge_workload_namespace}-producer"
          image   = var.edge_producer_image
          command = [""]
        }
      }
    }
  }
}

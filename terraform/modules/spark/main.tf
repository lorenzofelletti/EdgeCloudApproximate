resource "kubernetes_namespace_v1" "this" {
  metadata {
    annotations = local.annotations
    name        = var.name
  }
}

resource "kubernetes_service_account_v1" "spark" {
  metadata {
    annotations = local.annotations
    name        = local.service_account_name
    namespace   = kubernetes_namespace_v1.this.metadata[0].name
  }
}

resource "kubernetes_service_v1" "spark_master" {
  metadata {
    name      = "${var.name}-master"
    namespace = kubernetes_namespace_v1.this.metadata[0].name
    labels    = local.master_labels
  }
  spec {
    port {
      name        = "webui"
      port        = 8080
      target_port = 8080
    }
    port {
      name        = "spark"
      port        = 7077
      target_port = 7077
    }
    port {
      name        = "master-rest"
      port        = 6066
      target_port = 6066
    }
    selector   = local.master_labels
    cluster_ip = "None" # Headless service
  }
}

resource "kubernetes_service_v1" "spark_client" {
  metadata {
    name      = "${var.name}-client"
    namespace = kubernetes_namespace_v1.this.metadata[0].name
    labels    = local.client_labels
  }
  spec {
    selector   = local.client_labels
    cluster_ip = "None" # Headless service
  }

}

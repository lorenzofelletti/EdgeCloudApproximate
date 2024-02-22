locals {
  annotations = {
    "${var.app_label_key}" = var.name
  }
  labels = {
    "${var.app_label_key}" = var.name
  }

  service_account_name = coalesce(var.service_account_name, var.name)

  kafka_ctrl_svc_port = 29093

  kafka_quorum_voters = join(",", [
    for i in range(0, var.replicas) : "${i}@${var.name}-${i}.${kubernetes_service_account_v1.this.metadata[0].name}.kafka.svc.cluster.local:${local.kafka_ctrl_svc_port}"
  ])
}

resource "random_string" "cluster_id" {
  length  = 16
  special = false
}

resource "kubernetes_namespace_v1" "this" {
  metadata {
    annotations = local.annotations
    name        = var.name
  }
}

resource "kubernetes_service_account_v1" "this" {
  metadata {
    annotations = local.annotations
    name        = local.service_account_name
    namespace   = kubernetes_namespace_v1.this.metadata[0].name
  }
}

resource "kubernetes_service_v1" "this" {
  metadata {
    annotations = local.annotations
    labels      = local.labels
    name = try(
      replace(
        var.service_name_template,
        "{{name}}",
        var.name
      ), var.name
    )
    namespace = kubernetes_namespace_v1.this.metadata[0].name
  }
  spec {
    cluster_ip              = "None"
    cluster_ips             = ["None"]
    internal_traffic_policy = "Cluster"
    ip_families             = ["IPv4"]
    ip_family_policy        = "SingleStack"
    port {
      name     = "tcp-kafka-int"
      port     = 9092
      protocol = "TCP"
    }
    port {
      name     = "tcp-kafka-ctrl"
      port     = local.kafka_ctrl_svc_port
      protocol = "TCP"
    }
    selector         = local.labels
    session_affinity = "None"
    type             = "ClusterIP"
  }
}

resource "kubernetes_stateful_set_v1" "kafka" {
  metadata {
    annotations = local.annotations
    labels      = local.labels
    name        = var.name
    namespace   = kubernetes_namespace_v1.this.metadata[0].name
  }
  spec {
    pod_management_policy  = "Parallel"
    replicas               = var.replicas
    revision_history_limit = 30
    selector {
      match_labels = local.labels
    }
    service_name = kubernetes_service_v1.this.metadata[0].name
    template {
      metadata {
        annotations = local.annotations
        labels      = local.labels
      }
      spec {
        service_account_name = kubernetes_service_account_v1.this.metadata[0].name
        container {
          image = var.image
          name  = var.name
          command = [
            "sh",
            "-exc",
            <<-EOT
            export KAFKA_CFG_NODE_ID=$${HOSTNAME##*-}

            exec /opt/bitnami/scripts/kafka/run.sh
            EOT
          ]
          env {
            name  = "KAFKA_CFG_DELETE_TOPIC_ENABLE"
            value = "true"
          }
          env {
            name  = "ALLOW_PLAINTEXT_LISTENER"
            value = "yes"
          }
          env {
            name  = "KAFKA_CFG_PROCESS_ROLES"
            value = "controller,broker"
          }
          env {
            name  = "KAFKA_CFG_CONTROLLER_QUORUM_VOTERS"
            value = ""
          }
          env {
            name  = "KAFKA_KRAFT_CLUSTER_ID"
            value = random_string.cluster_id.result
          }
          env {
            name  = "KAFKA_CFG_LISTENERS"
            value = "PLAINTEXT://:9092,CONTROLLER://:${local.kafka_ctrl_svc_port}"
          }
          env {
            name  = "KAFKA_CFG_ADVERTISED_LISTENERS"
            value = "PLAINTEXT://:9092"
          }
          env {
            name  = "KAFKA_CFG_LISTENER_SECURITY_PROTOCOL_MAP"
            value = "PLAINTEXT:PLAINTEXT,CONTROLLER:PLAINTEXT"
          }
          env {
            name  = "KAFKA_CFG_CONTROLLER_LISTENER_NAMES"
            value = "CONTROLLER"
          }
          env {
            name  = "KAFKA_CFG_INTER_BROKER_LISTENER_NAME"
            value = "PLAINTEXT"
          }
          env {
            name  = "KAFKA_CFG_TOPIC_REPLICATION_FACTOR"
            value = var.replicas
          }
          env {
            name  = "KAFKA_CFG_TRANSACTION_STATE_LOG_REPLICATION_FACTOR"
            value = var.replicas
          }
          env {
            name  = "KAFKA_CFG_TRANSACTION_STATE_LOG_MIN_ISR"
            value = "1"
          }
        }
      }
    }
  }
}

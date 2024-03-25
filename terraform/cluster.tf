resource "minikube_cluster" "this" {
  count             = var.create_cluster ? 1 : 0
  driver            = "docker"
  container_runtime = "docker"
  cluster_name      = var.cluster_name
  memory            = var.cluster_specs.memory
  cpus              = var.cluster_specs.cpus
  addons = [
    "metrics-server",
    "dashboard",
    "default-storageclass",
    "storage-provisioner",
  ]

  # lifecycle {
  #   replace_triggered_by = [
  #     terraform_data.is_cluster_running
  #   ]
  # }
}

# TODO: reimplement this with a data external
# resource "terraform_data" "is_cluster_running" {
#   triggers_replace = [
#     timestamp()
#   ]

#   input = {
#     cluster_name = minikube_cluster.this[0].cluster_name
#   }

#   provisioner "local-exec" {
#     command = <<-EOT
#     if [[ "$(minikube profile list -o='json' | jq '.valid | .[] | select(.Name=="${var.cluster_name}") | .Status')" == '"Running"' ]] || false; then
#       echo "true"
#     else
#       echo "false"
#     fi
#     EOT
#   }
# }

resource "terraform_data" "load_images" {
  triggers_replace = [
    length(minikube_cluster.this) > 0 ? minikube_cluster.this[0].cluster_name : "",
    local.images_to_load,
  ]

  provisioner "local-exec" {
    command = length(minikube_cluster.this) > 0 ? join("\n", concat(["minikube profile ${minikube_cluster.this[0].cluster_name}\n"], [for image in local.images_to_load : "minikube image load ${image}"])) : ""
  }

  depends_on = [minikube_cluster.this, docker_image.edge, docker_image.spark_master, docker_image.spark_worker]
}

moved {
  from = minikube_cluster.this
  to   = minikube_cluster.this[0]
}

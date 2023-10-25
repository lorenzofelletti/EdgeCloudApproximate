resource "minikube_cluster" "this" {
  driver       = "docker"
  cluster_name = var.cluster_name
  addons = [
    "metrics-server",
    "dashboard",
    "default-storageclass",
    "storage-provisioner",
  ]
}

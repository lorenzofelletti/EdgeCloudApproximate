terraform {
  required_version = "~> 1.6.0"

  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0.2"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.23.0"
    }
    minikube = {
      source  = "scott-the-programmer/minikube"
      version = "~> 0.3.5"
    }
  }
}

provider "docker" {
  host = "unix:///var/run/docker.sock"
}

provider "kubernetes" {
  host = minikube_cluster.this.host

  client_certificate     = minikube_cluster.this.client_certificate
  client_key             = minikube_cluster.this.client_key
  cluster_ca_certificate = minikube_cluster.this.cluster_ca_certificate
}

provider "minikube" {
  kubernetes_version = "v1.27.4"
}

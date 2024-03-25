terraform {
  required_version = "~> 1.7.0"

  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.12.1"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.25"
    }
    minikube = {
      source  = "scott-the-programmer/minikube"
      version = "~> 0.3.10"
    }
  }
}

provider "docker" {
  host = "unix:///var/run/docker.sock"
}

provider "helm" {
  kubernetes {
    config_path    = "~/.kube/config"
    config_context = minikube_cluster.this.cluster_name
  }
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

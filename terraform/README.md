# Terraform Setup
This directory contains the Terraform configuration to run project on minikube (locally).

It is a better alternative to using Docker Compose, as it is more flexible and can be used to deploy to any Kubernetes cluster with minimal changes.

## Prerequisites
You must have the following installed:
- Terraform
- Minikube
- Docker

## Usage
0. Ensure that you have the prerequisites installed, and that Docker is running
1. Start minikube: `minikube start --driver=docker --download-only`
2. Run `terraform init` to initialize Terraform
3. Run `terraform plan -out=out.plan -var-file=dev.tfvars` to plan the deployment
4. Run `terraform apply out.plan` to apply the deployment
5. Wait for the deployment to finish
6. Run `minikube profile list` to get the name of the profile (default is `edge-cloud-approximate`)
7. Run `minikube profile <PROFILE_NAME>` to switch to the profile.

### Useful Commands
- `minikube dashboard` - Opens the Kubernetes dashboard
- `terraform destroy -var-file=dev.tfvars` - Destroys the deployment

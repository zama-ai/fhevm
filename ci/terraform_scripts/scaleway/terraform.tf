terraform {
  required_providers {
    scaleway = {
      source  = "scaleway/scaleway"
      version = "~> 2.73"
    }
  }
  required_version = "~> 1.14"
}

provider "scaleway" {
  zone = "fr-par-2"
}

# Provided through the Slab server environment.
variable "project_id" {
  type        = string
  description = "Scaleway project ID to attach the instance to"
}

# Provided through ci/slab.toml.
variable "instance_type" {
  type        = string
  description = "Scaleway instance type"
}

# Provided by the Slab server.
variable "instance_label" {
  type        = string
  description = "Instance name displayed in the Scaleway console"
}

# Provided by the Slab server.
variable "user_data" {
  type        = string
  description = "Cloud-init script run when the instance starts"
}

resource "scaleway_instance_ip" "github_runner" {
  project_id = var.project_id
}

data "scaleway_instance_security_group" "github_runner" {
  project_id = var.project_id
  name       = "github-actions-runner"
}

data "scaleway_instance_image" "gpu_benchmark_image" {
  name = "tfhe-rs-ubuntu-24-cuda"
}

resource "scaleway_instance_server" "scaleway_gpu_instance" {
  project_id = var.project_id
  image      = data.scaleway_instance_image.gpu_benchmark_image.id
  type       = var.instance_type
  name       = var.instance_label

  root_volume {
    size_in_gb = 200
  }

  ip_id = scaleway_instance_ip.github_runner.id

  user_data = {
    "cloud-init" = var.user_data
  }

  security_group_id = data.scaleway_instance_security_group.github_runner.id
}

output "instance_id" {
  value       = scaleway_instance_server.scaleway_gpu_instance.id
  description = "Unique ID of the Scaleway instance"
}

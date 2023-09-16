// Provider at runtime
variable "credentials_path" {
    type = string
    description = "Service account json file"
}

variable "project" {
    type = string
    description = "The project ID to deploy to"
}

variable "region" {
    type = string
    description = "GCP region"
}

variable "zone" {
    type = string
    description = "The primary zone where the bastion host will live"
}

variable "ssh_user" {
    type = string
    description = "SSH username"
}

variable "ssh_public_key" {
    type = string
    description = "SSH public key file path"
}

// Default values

variable "network_name" {
  type = string
  description = "Network name"
  default     = "management"
}

variable "public_subnets_count" {
  type = number
  description = "Number of public subnets"
  default = 1
}

variable "machine_type" {
    type = string
    description = "Instance type for the Jenkins host"
    default = "n1-standard-1"
}

variable "machine_image" {
    type = string
    description = "Machine image for jenkins host"
    default = "jenkins-master-v22041"
}

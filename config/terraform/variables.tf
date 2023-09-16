// Provider at runtime
variable "credentials_path" {
    type = string
    description = "Service account json file"
    default = "../secrets/winged-ratio-399207-9c8ec20121c8.json"
}

variable "project" {
    type = string
    description = "The project ID to deploy to"
    default = "winged-ratio-399207"
}

variable "region" {
    type = string
    description = "GCP region"
    default = "us-west1"
}

variable "zone" {
    type = string
    description = "The primary zone where the bastion host will live"
    default = "us-west1-b"
}

variable "ssh_user" {
    type = string
    description = "SSH username"
    default = "exch"
}

variable "ssh_public_key" {
    type = string
    description = "SSH public key file path"
    default = "../secrets/id_rsa.pub"
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
    default = "exch-2023-09-16"
}

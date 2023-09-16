resource "google_compute_network" "management" {
  name = var.network_name
  auto_create_subnetworks = false
  routing_mode = "REGIONAL"
}

resource "google_compute_subnetwork" "public_subnets" {
  count = var.public_subnets_count
  name          = "public-10-0-${count.index * 2 + 1}-0"
  ip_cidr_range = "10.0.${count.index * 2 + 1}.0/24"
  region        = var.region
  network       = google_compute_network.management.self_link
}

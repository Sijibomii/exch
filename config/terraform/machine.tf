resource "google_compute_firewall" "allow_ssh_to_jenkins" {
  project = var.project
  name    = "allow-ssh"
  network = google_compute_network.management.self_link

  allow {
    protocol = "tcp"
    ports    = ["22"]
  }
  source_ranges = ["0.0.0.0/0"]
  source_tags = ["ssh-allowed"]
}

resource "google_compute_address" "static" {
  name = "ipv4-address"
}


resource "google_compute_firewall" "allow_access_to_api" {
  project = var.project
  name    = "allow-access-to-exch-api"
  network = google_compute_network.management.self_link

  allow {
    protocol = "tcp"
    ports    = ["4001"]
  }

  source_ranges = ["0.0.0.0/0"]

  source_tags = ["exch-api"]
}
 

resource "google_compute_firewall" "allow_access_to_websocket" {
  project = var.project
  name    = "allow-access-to-websocket"
  network = google_compute_network.management.self_link

  allow {
    protocol = "tcp"
    ports    = ["6000"]
  }

  source_ranges = ["0.0.0.0/0"]

  source_tags = ["exch-websocket"]
}


resource "google_compute_instance" "worker_machine" {
  project      = var.project
  name         = "worker-machine"
  machine_type = var.machine_type
  zone         = var.zone

  tags = ["ssh-allowed", "exch-api", "exch-websocket"]


  boot_disk {
    initialize_params {
      image = var.machine_image
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.public_subnets[0].self_link 
    access_config {
      nat_ip = google_compute_address.static.address
    }
  }

  metadata = {
    ssh-keys = "${var.ssh_user}:${file(var.ssh_public_key)}"
  }
}

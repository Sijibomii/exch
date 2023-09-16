output "machine" {
    value = "${google_compute_instance.worker_machine.network_interface.0.access_config.0.nat_ip}"
} 
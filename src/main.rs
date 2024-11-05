mod proxctl;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Proxmox-Serverdaten
    let proxmox_url = "https://192.168.2.173:8006/api2/json";
    let username = "root@pam"; // z.B. "root@pam"
    let password = "";
    let node_name = "proxmox1"; // Name des Nodes im Proxmox-Cluster
    let vm_id = "103"; // ID der neuen VM

    let pc = proxctl::Proxctl::new(proxmox_url, username, password, node_name, vm_id);
    pc.create_new_vm().await;

    Ok(())
}
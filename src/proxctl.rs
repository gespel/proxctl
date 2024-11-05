use reqwest::Client;
use serde_json::json;

pub(crate) struct Proxctl {
    prox_url: String,
    username: String,
    password: String,
    node_name: String,
    vm_id: String,
}

impl Proxctl {
    pub(crate) fn new(prox_url: &str, username: &str, password: &str, node_name: &str, vm_id: &str) -> Proxctl {
        Proxctl {
            prox_url: prox_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            node_name: node_name.to_string(),
            vm_id: vm_id.to_string(),
        }
    }

    pub(crate) async fn create_new_vm(&self) {
        let client = Client::builder()
            .danger_accept_invalid_certs(true) // Nur für Tests, nicht für die Produktion empfohlen
            .build().unwrap();

        // Schritt 1: Authentifizierung
        let auth_url = format!("{}/access/ticket", self.prox_url);
        let auth_response = client
            .post(&auth_url)
            .form(&[("username", self.username.clone()), ("password", self.password.clone())])
            .send()
            .await.unwrap();

        let auth_data = auth_response.json::<serde_json::Value>().await.unwrap();
        let ticket = auth_data["data"]["ticket"].as_str().ok_or("Kein Ticket erhalten").unwrap();
        let csrf_token = auth_data["data"]["CSRFPreventionToken"].as_str().ok_or("Kein CSRF-Token erhalten").unwrap();

        // Schritt 2: Parameter für die neue VM festlegen
        let new_vm_data = json!({
            "vmid": self.vm_id.clone(),
            "name": "tester",
            "memory": 2048,            // Arbeitsspeicher in MB
            "sockets": 1,              // Anzahl der Sockets
            "cores": 2,                // Anzahl der CPU-Kerne pro Socket
            "storage": "local-lvm",        // Speichername (z.B. "local")
            "ide0": "local:iso/Fedora-Server-netinst-x86_64-41-1.4.iso,media=cdrom",  // Pfad zur ISO-Datei im Speicher
            "ide1": "local-lvm:32",        // Festplattengröße in GB
            "net0": "virtio,bridge=vmbr0" // Netzwerkgerät und Bridge
        });

        // Schritt 3: Neue VM erstellen
        let create_vm_url = format!("{}/nodes/{}/qemu", self.prox_url, self.node_name);
        let create_vm_response = client
            .post(&create_vm_url)
            .header("CSRFPreventionToken", csrf_token)
            .header("Cookie", format!("PVEAuthCookie={}", ticket))
            .json(&new_vm_data)
            .send()
            .await.unwrap();

        if create_vm_response.status().is_success() {
            println!("Neue VM erfolgreich erstellt.");
        } else {
            let error_message: serde_json::Value = create_vm_response.json().await.unwrap();
            println!("Fehler beim Erstellen der VM: {:?}", error_message);
        }
    }
}
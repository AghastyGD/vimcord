use std::process::Command;

pub fn resolve_host() -> String {
    if let Ok(host) = std::env::var("VIMCORD_HOST") {
        return host;
    }
    
    if is_wsl() {
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg("ip route | grep default | awk '{print $3'")
            .output()
        {
            let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ip.is_empty() {
                return ip;
            }
        }
    }

    "127.0.0.1".into()
}

fn is_wsl() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|v| v.contains("Microsoft"))
        .unwrap_or(false)
}
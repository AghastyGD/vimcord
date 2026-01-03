use std::fs;

pub fn resolve_host() -> String {
    if let Ok(host) = std::env::var("VIMCORD_HOST") {

        return host;
    }
    
    if is_wsl() {
        if let Some(ip) = wsl_default_gateway() {
            return ip;
        }
    }

    "127.0.0.1".into()
}

fn is_wsl() -> bool {
    std::env::var("WSL_DISTRO_NAME").is_ok()
}

fn wsl_default_gateway() -> Option<String> {
    let contents = fs::read_to_string("/proc/net/route").ok()?;

    for line in contents.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 3 {
            continue;
        }

        if fields[1] == "00000000" {
            let hex = fields[2];

            if let Ok(gateway) = u32::from_str_radix(hex, 16) {
                let bytes = gateway.to_le_bytes();
                return Some(format!(
                    "{}.{}.{}.{}",
                    bytes[0], bytes[1], bytes[2], bytes[3]
                ));
            }
        }
    }

    None
}

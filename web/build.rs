#![feature(format_args_capture)]

fn main() {
    if let Ok(contents) = std::fs::read_to_string("../.env") {
        for line in contents.lines() {
            if line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('=').collect();
            match (parts.get(0), parts.get(1)) {
                (Some(k), Some(v)) => std::env::set_var(k, v),
                _ => continue,
            }
        }
    }

    let addr = std::env::var("JIRS_SERVER_BIND").unwrap_or("0.0.0.0".to_string());
    let addr = if addr.as_str() == "0.0.0.0" || addr.as_str() == "localhost" {
        "localhost"
    } else {
        addr.as_str()
    }
    .to_string();
    let port = std::env::var("JIRS_SERVER_PORT").unwrap_or("80".to_string());
    let port = match port.as_str() {
        "80" | "8080" | "443" => "".to_string(),
        _ => format!(":{}", port),
    };
    let addr = format!("{}{}", addr, port);

    std::fs::write(
        "./src/location.rs",
        format!(
            "
pub fn host_url() -> &'static str {{
    if cfg!(debug_assertions) {{
        \"http://{addr}\"
    }} else {{
        \"https://{addr}\"
    }}
}}
pub fn ws_url() -> &'static str {{
    if cfg!(debug_assertions) {{
        \"ws://{addr}/ws/\"
    }} else {{
        \"wss://{addr}/ws/\"
    }}
}}
    ",
            addr = addr
        )
        .trim(),
    )
    .unwrap();
}

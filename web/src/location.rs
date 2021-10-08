static mut HOST: String = String::new();
static mut WS: String = String::new();

fn ensure_host() {
    unsafe {
        if HOST.is_empty() {
            HOST = format!(
                "{}//{}",
                seed::window().location().protocol().unwrap(),
                seed::window().location().host().unwrap()
            );
            let host: String = seed::window().location().host().unwrap();
            let is_local = host.ends_with("lvh.me")
                || host.contains("localhost")
                || host.starts_with("127.")
                || host.contains("0.0.0.0");
            WS = format!(
                "{}//{}/ws/",
                match seed::window().location().protocol().unwrap().as_str() {
                    "http:" => "ws:",
                    _ => "wss:",
                },
                if is_local {
                    "localhost:5000"
                } else {
                    host.as_str()
                }
            );
        }
    }
}

pub fn host_url() -> &'static str {
    ensure_host();
    unsafe { HOST.as_str() }
}

pub fn ws_url() -> &'static str {
    ensure_host();
    unsafe { WS.as_str() }
}

pub fn host_url() -> &'static str {
    if cfg!(debug_assertions) {
        "http://localhost:5000"
    } else {
        "https://localhost:5000"
    }
}
pub fn ws_url() -> &'static str {
    if cfg!(debug_assertions) {
        "ws://localhost:5000/ws/"
    } else {
        "wss://localhost:5000/ws/"
    }
}
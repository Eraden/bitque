use jirs_data::*;

pub fn send_ws_msg(msg: WsMsg) {
    use crate::send_bin_code;
    use wasm_bindgen::JsValue;

    let binary = bincode::serialize(&msg).unwrap();
    let data = JsValue::from_serde(&binary).unwrap();
    send_bin_code(data);
}

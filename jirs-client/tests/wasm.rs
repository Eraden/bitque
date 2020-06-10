extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use jirs_client::validations::*;

#[wasm_bindgen_test]
fn test_is_token() {
    assert_eq!(is_email("foo@"), false);
    assert_eq!(is_email("foo@bar"), false);
    assert_eq!(is_email("foo@bar."), false);
    assert_eq!(is_email("foo@bar.com"), true);
}

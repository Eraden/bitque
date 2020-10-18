use syntect::parsing::SyntaxSet;

static mut SET: Option<SyntaxSet> = None;

pub fn load() -> &'static SyntaxSet {
    if unsafe { SET.as_ref() }.is_none() {
        unsafe { SET = serde_cbor::from_slice(include_bytes!("./syntax_set.cbor")).ok() };
    }
    unsafe { SET.as_ref() }.unwrap()
}

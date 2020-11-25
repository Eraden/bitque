
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Authorized Keys".to_string(),
  file_extensions: vec!["authorized_keys".to_string(),"pub".to_string()],
  scope: Scope { a: 282149286576128, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_value-options_0".to_string(), ContextId { index: 10206 });
    v.insert("#anon_value-options_3".to_string(), ContextId { index: 10209 });
    v.insert("#anon_value-options_4".to_string(), ContextId { index: 10210 });
    v.insert("#anon_value-options_2".to_string(), ContextId { index: 10208 });
    v.insert("#anon_value-options_6".to_string(), ContextId { index: 10212 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 10204 });
    v.insert("__start".to_string(), ContextId { index: 10216 });
    v.insert("main".to_string(), ContextId { index: 10218 });
    v.insert("#anon_value-options_1".to_string(), ContextId { index: 10207 });
    v.insert("#anon_value-options_8".to_string(), ContextId { index: 10214 });
    v.insert("flag-options".to_string(), ContextId { index: 10217 });
    v.insert("value-option-body".to_string(), ContextId { index: 10220 });
    v.insert("#anon_value-options_5".to_string(), ContextId { index: 10211 });
    v.insert("value-options".to_string(), ContextId { index: 10221 });
    v.insert("strings".to_string(), ContextId { index: 10219 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 10205 });
    v.insert("#anon_value-options_7".to_string(), ContextId { index: 10213 });
    v.insert("__main".to_string(), ContextId { index: 10215 });
    v
  }
} }
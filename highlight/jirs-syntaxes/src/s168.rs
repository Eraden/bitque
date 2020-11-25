
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Known Hosts".to_string(),
  file_extensions: vec!["known_hosts".to_string()],
  scope: Scope { a: 282153581543424, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_hostname-or-ip-value_0".to_string(), ContextId { index: 10222 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 10224 });
    v.insert("hostname-or-ip-value".to_string(), ContextId { index: 10227 });
    v.insert("main".to_string(), ContextId { index: 10228 });
    v.insert("__start".to_string(), ContextId { index: 10226 });
    v.insert("__main".to_string(), ContextId { index: 10225 });
    v.insert("#anon_hostname-or-ip-value_1".to_string(), ContextId { index: 10223 });
    v
  }
} }
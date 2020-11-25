
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Java Properties".to_string(),
  file_extensions: vec!["properties".to_string()],
  scope: Scope { a: 844605318758400, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_property_1".to_string(), ContextId { index: 2429 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2426 });
    v.insert("#anon_property-value_0".to_string(), ContextId { index: 2427 });
    v.insert("backslash".to_string(), ContextId { index: 2432 });
    v.insert("__start".to_string(), ContextId { index: 2431 });
    v.insert("__main".to_string(), ContextId { index: 2430 });
    v.insert("property-value".to_string(), ContextId { index: 2435 });
    v.insert("property".to_string(), ContextId { index: 2434 });
    v.insert("#anon_property_0".to_string(), ContextId { index: 2428 });
    v.insert("main".to_string(), ContextId { index: 2433 });
    v
  }
} }
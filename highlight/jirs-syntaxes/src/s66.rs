
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML (Rails)".to_string(),
  file_extensions: vec!["rails".to_string(),"rhtml".to_string(),"erb".to_string(),"html.erb".to_string()],
  scope: Scope { a: 281496455872512, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 4699 });
    v.insert("main".to_string(), ContextId { index: 4703 });
    v.insert("__main".to_string(), ContextId { index: 4701 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4698 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 4700 });
    v.insert("__start".to_string(), ContextId { index: 4702 });
    v
  }
} }
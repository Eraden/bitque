
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SQL (Rails)".to_string(),
  file_extensions: vec!["erbsql".to_string(),"sql.erb".to_string()],
  scope: Scope { a: 844721287200768, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 4739 });
    v.insert("__main".to_string(), ContextId { index: 4737 });
    v.insert("__start".to_string(), ContextId { index: 4738 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 4736 });
    v
  }
} }
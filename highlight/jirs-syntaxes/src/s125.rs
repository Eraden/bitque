
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "INI".to_string(),
  file_extensions: vec!["ini".to_string(),"INI".to_string(),"INF".to_string(),"reg".to_string(),"REG".to_string(),"lng".to_string(),"cfg".to_string(),"CFG".to_string(),"desktop".to_string(),"url".to_string(),"URL".to_string(),".editorconfig".to_string(),".hgrc".to_string(),"hgrc".to_string()],
  scope: Scope { a: 844931736272896, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 7732 });
    v.insert("__start".to_string(), ContextId { index: 7731 });
    v.insert("__main".to_string(), ContextId { index: 7730 });
    v
  }
} }
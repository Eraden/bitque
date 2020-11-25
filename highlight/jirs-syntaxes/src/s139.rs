
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "passwd".to_string(),
  file_extensions: vec!["passwd".to_string()],
  scope: Scope { a: 844983275880448, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("directory".to_string(), ContextId { index: 8561 });
    v.insert("gid".to_string(), ContextId { index: 8562 });
    v.insert("comment".to_string(), ContextId { index: 8560 });
    v.insert("__main".to_string(), ContextId { index: 8558 });
    v.insert("uid".to_string(), ContextId { index: 8566 });
    v.insert("__start".to_string(), ContextId { index: 8559 });
    v.insert("main".to_string(), ContextId { index: 8563 });
    v.insert("password".to_string(), ContextId { index: 8564 });
    v.insert("shell".to_string(), ContextId { index: 8565 });
    v
  }
} }
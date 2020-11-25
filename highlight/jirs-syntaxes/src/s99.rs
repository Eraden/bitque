
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CpuInfo".to_string(),
  file_extensions: vec!["cpuinfo".to_string()],
  scope: Scope { a: 844824362090496, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 6601 });
    v.insert("main".to_string(), ContextId { index: 6603 });
    v.insert("__start".to_string(), ContextId { index: 6602 });
    v
  }
} }
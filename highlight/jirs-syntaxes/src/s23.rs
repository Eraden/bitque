
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Log".to_string(),
  file_extensions: vec!["gitlog".to_string()],
  scope: Scope { a: 281565172858880, b: 0 },
  first_line_match: Some("^commit\\s+\\h{7,}".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_0".to_string(), ContextId { index: 1816 });
    v.insert("__start".to_string(), ContextId { index: 1819 });
    v.insert("#anon_commit-header_0".to_string(), ContextId { index: 1815 });
    v.insert("main".to_string(), ContextId { index: 1821 });
    v.insert("commit-header".to_string(), ContextId { index: 1820 });
    v.insert("prototype".to_string(), ContextId { index: 1822 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 1817 });
    v.insert("__main".to_string(), ContextId { index: 1818 });
    v
  }
} }
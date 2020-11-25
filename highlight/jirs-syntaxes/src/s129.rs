
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "jsonnet".to_string(),
  file_extensions: vec!["jsonnet".to_string(),"libsonnet".to_string(),"libjsonnet".to_string()],
  scope: Scope { a: 844940326207488, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("single_string".to_string(), ContextId { index: 7881 });
    v.insert("unquoted".to_string(), ContextId { index: 7883 });
    v.insert("main".to_string(), ContextId { index: 7880 });
    v.insert("string".to_string(), ContextId { index: 7882 });
    v.insert("__main".to_string(), ContextId { index: 7877 });
    v.insert("comment".to_string(), ContextId { index: 7879 });
    v.insert("__start".to_string(), ContextId { index: 7878 });
    v
  }
} }

#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "JSON (Terraform)".to_string(),
  file_extensions: vec!["tfstate".to_string()],
  scope: Scope { a: 844588148654080, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 9345 });
    v.insert("main".to_string(), ContextId { index: 9346 });
    v.insert("__main".to_string(), ContextId { index: 9344 });
    v
  }
} }
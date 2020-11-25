
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Git Ignore".to_string(),
  file_extensions: vec!["exclude".to_string(),"gitignore".to_string(),".gitignore".to_string()],
  scope: Scope { a: 281565172727808, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 1803 });
    v.insert("pattern-content".to_string(), ContextId { index: 1804 });
    v.insert("__main".to_string(), ContextId { index: 1801 });
    v.insert("__start".to_string(), ContextId { index: 1802 });
    v
  }
} }
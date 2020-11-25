
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "MultiMarkdown".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 281496454758450, b: 0 },
  first_line_match: Some("(?i)^format:\\s*complete\\s*$".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("header".to_string(), "((?=[A-Za-z0-9])[\\w -]+)(:)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 3373 });
    v.insert("main".to_string(), ContextId { index: 3375 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 3372 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 3371 });
    v.insert("__start".to_string(), ContextId { index: 3374 });
    v
  }
} }
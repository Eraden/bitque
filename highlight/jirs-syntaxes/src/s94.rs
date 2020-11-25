
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "CMakeCache".to_string(),
  file_extensions: vec!["CMakeCache.txt".to_string()],
  scope: Scope { a: 844807182221312, b: 0 },
  first_line_match: Some("# This is the CMakeCache file.".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "[a-zA-Z0-9\\-_:\\\\/ .]+".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("main".to_string(), ContextId { index: 6262 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 6255 });
    v.insert("expect-type".to_string(), ContextId { index: 6259 });
    v.insert("expect-value".to_string(), ContextId { index: 6260 });
    v.insert("__main".to_string(), ContextId { index: 6256 });
    v.insert("comments".to_string(), ContextId { index: 6258 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 6254 });
    v.insert("key-value-pair".to_string(), ContextId { index: 6261 });
    v.insert("__start".to_string(), ContextId { index: 6257 });
    v
  }
} }
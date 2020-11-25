
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "varlink".to_string(),
  file_extensions: vec!["varlink".to_string()],
  scope: Scope { a: 845129304768512, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "([A-Z][a-zA-Z0-9_]*)".to_string());
    v.insert("interface_name".to_string(), "([a-z](\\-*[a-z0-9])*(\\.[a-z0-9](\\-*[a-z0-9])*)+)".to_string());
    v.insert("field_name".to_string(), "[A-Za-z]([_]?[A-Za-z0-9])*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("statements".to_string(), ContextId { index: 10341 });
    v.insert("block-inner".to_string(), ContextId { index: 10337 });
    v.insert("method-return".to_string(), ContextId { index: 10340 });
    v.insert("type".to_string(), ContextId { index: 10342 });
    v.insert("block".to_string(), ContextId { index: 10336 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 10333 });
    v.insert("main".to_string(), ContextId { index: 10339 });
    v.insert("__main".to_string(), ContextId { index: 10334 });
    v.insert("__start".to_string(), ContextId { index: 10335 });
    v.insert("comments".to_string(), ContextId { index: 10338 });
    v
  }
} }
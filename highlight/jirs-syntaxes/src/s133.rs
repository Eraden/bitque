
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Manpage".to_string(),
  file_extensions: vec!["man".to_string()],
  scope: Scope { a: 844957506076672, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("section_heading".to_string(), "^\\S.*$".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_body_0".to_string(), ContextId { index: 8332 });
    v.insert("__main".to_string(), ContextId { index: 8333 });
    v.insert("__start".to_string(), ContextId { index: 8334 });
    v.insert("body".to_string(), ContextId { index: 8335 });
    v.insert("main".to_string(), ContextId { index: 8337 });
    v.insert("first_line".to_string(), ContextId { index: 8336 });
    v
  }
} }
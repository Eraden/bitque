
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "camlp4".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844661156741120, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__main".to_string(), ContextId { index: 3566 });
    v.insert("camlpppp-streams".to_string(), ContextId { index: 3568 });
    v.insert("main".to_string(), ContextId { index: 3569 });
    v.insert("#anon_camlpppp-streams_0".to_string(), ContextId { index: 3564 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 3565 });
    v.insert("__start".to_string(), ContextId { index: 3567 });
    v
  }
} }
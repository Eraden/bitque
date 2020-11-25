
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Private Key".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 845107840417792, b: 0 },
  first_line_match: Some("^-----BEGIN [\\w ]+ PRIVATE KEY-----".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("__start".to_string(), ContextId { index: 10230 });
    v.insert("__main".to_string(), ContextId { index: 10229 });
    v.insert("main".to_string(), ContextId { index: 10231 });
    v
  }
} }
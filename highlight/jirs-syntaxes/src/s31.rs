
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Literate Haskell".to_string(),
  file_extensions: vec!["lhs".to_string()],
  scope: Scope { a: 281629597958179, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("global-braces".to_string(), ContextId { index: 2182 });
    v.insert("prototype".to_string(), ContextId { index: 2185 });
    v.insert("__start".to_string(), ContextId { index: 2181 });
    v.insert("haskell-code".to_string(), ContextId { index: 2183 });
    v.insert("#anon_global-braces_0".to_string(), ContextId { index: 2178 });
    v.insert("__main".to_string(), ContextId { index: 2180 });
    v.insert("#anon_haskell-code_0".to_string(), ContextId { index: 2179 });
    v.insert("main".to_string(), ContextId { index: 2184 });
    v
  }
} }
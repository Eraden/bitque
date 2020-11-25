
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SSH Crypto".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 282157887127552, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("ssh-ciphers".to_string(), ContextId { index: 10304 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 10298 });
    v.insert("main".to_string(), ContextId { index: 10303 });
    v.insert("ssh-key-types".to_string(), ContextId { index: 10306 });
    v.insert("ssh-kex-algorithms".to_string(), ContextId { index: 10305 });
    v.insert("ssh-mac-algorithms".to_string(), ContextId { index: 10307 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 10299 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 10297 });
    v.insert("__main".to_string(), ContextId { index: 10301 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 10300 });
    v.insert("__start".to_string(), ContextId { index: 10302 });
    v
  }
} }
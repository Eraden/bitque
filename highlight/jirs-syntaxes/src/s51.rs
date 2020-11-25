
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "OCamllex".to_string(),
  file_extensions: vec!["mll".to_string()],
  scope: Scope { a: 844652563398656, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 3526 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 3524 });
    v.insert("#anon_match-patterns_0".to_string(), ContextId { index: 3529 });
    v.insert("__main".to_string(), ContextId { index: 3532 });
    v.insert("actions".to_string(), ContextId { index: 3534 });
    v.insert("comments".to_string(), ContextId { index: 3536 });
    v.insert("match-patterns".to_string(), ContextId { index: 3538 });
    v.insert("strings".to_string(), ContextId { index: 3539 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 3523 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 3525 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 3527 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 3528 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 3531 });
    v.insert("__start".to_string(), ContextId { index: 3533 });
    v.insert("#anon_actions_0".to_string(), ContextId { index: 3522 });
    v.insert("#anon_match-patterns_1".to_string(), ContextId { index: 3530 });
    v.insert("main".to_string(), ContextId { index: 3537 });
    v.insert("chars".to_string(), ContextId { index: 3535 });
    v
  }
} }
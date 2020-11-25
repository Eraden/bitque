
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "OCamlyacc".to_string(),
  file_extensions: vec!["mly".to_string()],
  scope: Scope { a: 844656858365952, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_1".to_string(), ContextId { index: 3547 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 3546 });
    v.insert("declaration-matches".to_string(), ContextId { index: 3556 });
    v.insert("rule-patterns".to_string(), ContextId { index: 3560 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 3548 });
    v.insert("#anon_rules_0".to_string(), ContextId { index: 3550 });
    v.insert("references".to_string(), ContextId { index: 3559 });
    v.insert("#anon_symbol-types_0".to_string(), ContextId { index: 3552 });
    v.insert("semantic-actions".to_string(), ContextId { index: 3562 });
    v.insert("__start".to_string(), ContextId { index: 3554 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 3541 });
    v.insert("precs".to_string(), ContextId { index: 3558 });
    v.insert("#anon_declaration-matches_0".to_string(), ContextId { index: 3542 });
    v.insert("#anon_declaration-matches_1".to_string(), ContextId { index: 3543 });
    v.insert("#anon_declaration-matches_2".to_string(), ContextId { index: 3544 });
    v.insert("comments".to_string(), ContextId { index: 3555 });
    v.insert("rules".to_string(), ContextId { index: 3561 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 3540 });
    v.insert("#anon_declaration-matches_3".to_string(), ContextId { index: 3545 });
    v.insert("#anon_semantic-actions_0".to_string(), ContextId { index: 3551 });
    v.insert("__main".to_string(), ContextId { index: 3553 });
    v.insert("symbol-types".to_string(), ContextId { index: 3563 });
    v.insert("main".to_string(), ContextId { index: 3557 });
    v.insert("#anon_rule-patterns_0".to_string(), ContextId { index: 3549 });
    v
  }
} }
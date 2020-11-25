
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Regular Expressions (Javascript)".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844613911511040, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier".to_string(), "[_$[:alpha:]][_$[:alnum:]]*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("anchor".to_string(), ContextId { index: 2721 });
    v.insert("group-assertion".to_string(), ContextId { index: 2725 });
    v.insert("operator".to_string(), ContextId { index: 2728 });
    v.insert("#anon_group-definition_0".to_string(), ContextId { index: 2718 });
    v.insert("quantifier".to_string(), ContextId { index: 2729 });
    v.insert("#anon_group-assertion_0".to_string(), ContextId { index: 2717 });
    v.insert("__main".to_string(), ContextId { index: 2719 });
    v.insert("backref".to_string(), ContextId { index: 2722 });
    v.insert("#anon_character-class-definition_0".to_string(), ContextId { index: 2716 });
    v.insert("__start".to_string(), ContextId { index: 2720 });
    v.insert("character-class".to_string(), ContextId { index: 2723 });
    v.insert("character-class-definition".to_string(), ContextId { index: 2724 });
    v.insert("main".to_string(), ContextId { index: 2727 });
    v.insert("group-definition".to_string(), ContextId { index: 2726 });
    v
  }
} }
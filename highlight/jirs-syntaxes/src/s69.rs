
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Ruby on Rails".to_string(),
  file_extensions: vec!["rxml".to_string(),"builder".to_string()],
  scope: Scope { a: 844708402364416, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_late-expressions_7".to_string(), ContextId { index: 4727 });
    v.insert("__start".to_string(), ContextId { index: 4731 });
    v.insert("#anon_late-expressions_9".to_string(), ContextId { index: 4729 });
    v.insert("#anon_late-expressions_3".to_string(), ContextId { index: 4723 });
    v.insert("#anon_late-expressions_5".to_string(), ContextId { index: 4725 });
    v.insert("early-expressions".to_string(), ContextId { index: 4732 });
    v.insert("#anon_late-expressions_1".to_string(), ContextId { index: 4721 });
    v.insert("main".to_string(), ContextId { index: 4735 });
    v.insert("#anon_late-expressions_8".to_string(), ContextId { index: 4728 });
    v.insert("embedded-expressions".to_string(), ContextId { index: 4733 });
    v.insert("#anon_late-expressions_6".to_string(), ContextId { index: 4726 });
    v.insert("late-expressions".to_string(), ContextId { index: 4734 });
    v.insert("__main".to_string(), ContextId { index: 4730 });
    v.insert("#anon_late-expressions_0".to_string(), ContextId { index: 4720 });
    v.insert("#anon_late-expressions_2".to_string(), ContextId { index: 4722 });
    v.insert("#anon_late-expressions_4".to_string(), ContextId { index: 4724 });
    v
  }
} }
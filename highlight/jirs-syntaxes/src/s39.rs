
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "BibTeX".to_string(),
  file_extensions: vec!["bib".to_string()],
  scope: Scope { a: 281668250238976, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_4".to_string(), ContextId { index: 2734 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 2736 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2730 });
    v.insert("#anon_nested_braces_0".to_string(), ContextId { index: 2738 });
    v.insert("#anon_string_content_1".to_string(), ContextId { index: 2740 });
    v.insert("integer".to_string(), ContextId { index: 2743 });
    v.insert("string_content".to_string(), ContextId { index: 2746 });
    v.insert("#anon_string_content_0".to_string(), ContextId { index: 2739 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 2731 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 2733 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 2732 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 2735 });
    v.insert("__main".to_string(), ContextId { index: 2741 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 2737 });
    v.insert("__start".to_string(), ContextId { index: 2742 });
    v.insert("nested_braces".to_string(), ContextId { index: 2745 });
    v.insert("main".to_string(), ContextId { index: 2744 });
    v
  }
} }
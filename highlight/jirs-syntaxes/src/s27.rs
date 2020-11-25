
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Graphviz (DOT)".to_string(),
  file_extensions: vec!["dot".to_string(),"DOT".to_string(),"gv".to_string()],
  scope: Scope { a: 844562369085440, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("expressions".to_string(), ContextId { index: 1983 });
    v.insert("braces".to_string(), ContextId { index: 1979 });
    v.insert("#anon_embedded-html_2".to_string(), ContextId { index: 1975 });
    v.insert("main".to_string(), ContextId { index: 1984 });
    v.insert("#anon_embedded-html_1".to_string(), ContextId { index: 1974 });
    v.insert("comments".to_string(), ContextId { index: 1981 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 1969 });
    v.insert("__main".to_string(), ContextId { index: 1977 });
    v.insert("#anon_expressions_0".to_string(), ContextId { index: 1976 });
    v.insert("brackets".to_string(), ContextId { index: 1980 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 1970 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 1971 });
    v.insert("#anon_comments_2".to_string(), ContextId { index: 1972 });
    v.insert("__start".to_string(), ContextId { index: 1978 });
    v.insert("#anon_braces_0".to_string(), ContextId { index: 1968 });
    v.insert("embedded-html".to_string(), ContextId { index: 1982 });
    v.insert("#anon_embedded-html_0".to_string(), ContextId { index: 1973 });
    v
  }
} }

#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b (cycle) \\b \\s* ([A-Za-z_][A-Za-z_0-9]*)?)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453202484527104,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636636838758597,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632124632,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
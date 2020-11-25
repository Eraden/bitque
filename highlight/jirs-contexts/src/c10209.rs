
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:([^\"]+)(:))?(?:(\\d{1,5})|(\\*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444217419694237,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172957,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089176461530,
            b: 44191571343572992,
        },
    ]),(4, vec![
        Scope {
            a: 49259061555888285,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
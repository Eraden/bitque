
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
      regex: Regex::new("(?:(\\.\\.\\.)\\s*)?([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628265336982,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444204392382614,
            b: 0,
        },
        Scope {
            a: 49259087310291094,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
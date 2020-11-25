
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
      regex: Regex::new("(\\\\)(?m:$)\\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620736970765,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(?:\\\\|[abefnrtv\\\'\"?]|[0-3][0-9]{0,2}|[4-7][0-9]?|x[a-fA-F0-9]+|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8})"),
      scope: vec![
        Scope {
            a: 59955200847314957,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\."),
      scope: vec![
        Scope {
            a: 50103314684903437,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
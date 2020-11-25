
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
      regex: Regex::new("\\(\\)"),
      scope: vec![
        Scope {
            a: 59955110657982569,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?{0,1})(``[\\p{L}0-9\'`^:,._ ]+``|[\\p{L}0-9\'`<>^._ ]\\w*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876845490176,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
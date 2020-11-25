
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
      regex: Regex::new("^([<]{7})\\s(.+)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314808375610,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 50103314666553387,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^([=|]{7})(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314808373440,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^([>]{7})\\s(.+)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314808375611,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 50103314666553387,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
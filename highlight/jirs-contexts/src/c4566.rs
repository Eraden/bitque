
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
      regex: Regex::new("(\\\\x\\h{2})|(\\\\[0-7]{1,3})|(\\\\[\\\\\"\'abfnrtv])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200847315536,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847315298,
            b: 17451448556060672,
        },
    ]),(3, vec![
        Scope {
            a: 59955200847315006,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\."),
      scope: vec![
        Scope {
            a: 50104723418644721,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
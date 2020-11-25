
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
      regex: Regex::new("(?xi:^ \\s* (go) \\s* (to) \\s+ (\\d+) )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453245434200064,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636636716337349,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636716337349,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130768963781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
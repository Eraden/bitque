
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
      regex: Regex::new("\\\\[1-9][0-9]*"),
      scope: vec![
        Scope {
            a: 52636787067453484,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\k)(<)([_$\\p{L}][_$\\p{L}\\p{N}]*)(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787067453484,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629318583033,
            b: 51228634739900416,
        },
    ]),(3, vec![
        Scope {
            a: 49259087299543084,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629318583033,
            b: 48132409996083200,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
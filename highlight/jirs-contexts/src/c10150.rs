
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
      regex: Regex::new("\\s*(<!--)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510878516084736,
            b: 0,
        },
        Scope {
            a: 47288629323038902,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?:(-->)\\s*)?(</)(script)(>)"),
      scope: vec![
        Scope {
            a: 46444230169723051,
            b: 1407374883553280,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510878516084736,
            b: 0,
        },
        Scope {
            a: 47288629323038891,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632122658,
            b: 1407374883553280,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324153003,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
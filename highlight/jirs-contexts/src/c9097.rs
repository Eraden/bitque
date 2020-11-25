
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
      regex: Regex::new("(#)(\\h{3}|\\h{6})\\b"),
      scope: vec![
        Scope {
            a: 59955136441680405,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004814,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#)(\\h{4}|\\h{8})\\b"),
      scope: vec![
        Scope {
            a: 59955136441680406,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004814,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
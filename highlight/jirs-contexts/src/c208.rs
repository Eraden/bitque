
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
      regex: Regex::new("\\b(0[xX])\\h*\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 2533274790395904,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 2533274790395904,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0)[0-7]+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 2533274790395904,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070690,
            b: 2533274790395904,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:0|[1-9][0-9]*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 2533274790395904,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
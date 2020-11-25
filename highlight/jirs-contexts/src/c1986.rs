
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
      regex: Regex::new("(?m:$)|(?=\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(extends)\\s+([a-zA-Z0-9_\\.]+(?:<(?:[a-zA-Z0-9_, ])+>)?)\\s*"),
      scope: vec![
        Scope {
            a: 46444204394218189,
            b: 202099175012171776,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439040352289,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392186470432801,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(implements)\\s"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439070695457,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1987 }),
    ]),
      with_prototype: None
    }),
]
} }
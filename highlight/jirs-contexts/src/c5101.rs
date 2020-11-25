
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46445273856868421,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445273856868421,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\w+)|\'([^\']+)\'|\"([^\"]+)\"|`([^`]+)`)(?=[ \\t]*(?:[^\\w\'\"`. \\t]|(?m:$)))"),
      scope: vec![
        Scope {
            a: 46445273856868421,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615109,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615109,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615109,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615109,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\w+|\'[^\']+\'|\"[^\"]+\"|`[^`]+`)\\s*(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788226932805,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[ \\t]*(?:[^\\w\'\"`. \\t]|(?m:$)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
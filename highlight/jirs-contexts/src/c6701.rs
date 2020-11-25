
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
      regex: Regex::new(";"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288689446879232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b(void|bool|num|int|double|dynamic|var|String|List|Map)\\b|([a-zA-Z_][a-zA-Z0-9_]*))\\s+([a-zA-Z_][a-zA-Z0-9_]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576511352928,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 128917046873292800,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 128916449872838656,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6702 }),
    ]),
      with_prototype: None
    }),
]
} }
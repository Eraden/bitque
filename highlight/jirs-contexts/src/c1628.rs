
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
  prototype: Some(
    ContextId {
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\#(?=\\?{,2}(?:[_A-Za-z][_A-Za-z\\d@]*|\\\'((?:\\\\\\\\)*\\\\\\\'|[^\'\'])*\\\'))"),
      scope: vec![
        Scope {
            a: 47288629349777428,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1630 }),
        ContextReference::Direct(ContextId { index: 1632 }),
        ContextReference::Direct(ContextId { index: 1675 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[_A-Z][_A-Za-z\\d@]*(?=\\.\\?{,2}(?:[_A-Za-z][_A-Za-z\\d@]*|\\\'((?:\\\\\\\\)*\\\\\\\'|[^\'\'])*\\\'))"),
      scope: vec![
        Scope {
            a: 46445909496823828,
            b: 0,
        },
        Scope {
            a: 49259087293317120,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1630 }),
    ]),
      with_prototype: None
    }),
]
} }
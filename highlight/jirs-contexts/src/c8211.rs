
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
        a: 46443865086558208,
        b: 0,
    },
    Scope {
        a: 46444131382984827,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46443865086558208,
        b: 0,
    },
    Scope {
        a: 46444131382984827,
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
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 46443865086558208,
            b: 0,
        },
        Scope {
            a: 47288629318582443,
            b: 34621422135410688,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620737429518,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8277 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8276 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8275 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8311 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8304 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8297 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8298 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8303 })),
]
} }
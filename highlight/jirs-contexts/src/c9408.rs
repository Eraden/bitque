
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
      regex: Regex::new("\\}"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629325660310,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9582 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=:)\\s*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9409 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9625 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9616 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9598 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9677 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9699 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9559 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9651 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9567 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9560 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9580 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9653 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9654 })),
]
} }
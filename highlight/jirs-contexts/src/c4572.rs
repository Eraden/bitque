
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4554 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4556 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4617 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4631 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4618 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4602 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(await)\\b"),
      scope: vec![
        Scope {
            a: 52636787066208318,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4594 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4626 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4580 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4600 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4610 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4560 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4628 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4585 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 50103314682151348,
            b: 364228886151561216,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\]"),
      scope: vec![
        Scope {
            a: 50103314682151348,
            b: 364510361128271872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 50103314682151348,
            b: 273593943650729984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4605 })),
]
} }
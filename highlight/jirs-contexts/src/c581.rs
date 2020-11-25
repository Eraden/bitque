
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 612 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 624 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 613 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 659 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 625 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 557 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 556 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 674 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 560 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620721897472,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)|\\}"),
      scope: vec![
        Scope {
            a: 50103314683854860,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
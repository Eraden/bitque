
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
      regex: Regex::new("\\b(const|flip)\\b"),
      scope: vec![
        Scope {
            a: 61925255135625351,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8775 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8776 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8777 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8778 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8779 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8780 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8781 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8782 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8783 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8784 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8785 })),
]
} }
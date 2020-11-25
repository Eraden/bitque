
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
      regex: Regex::new("([\\]\\}]\\s*[:=])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628104773632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6600 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6595 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6599 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6598 })),
]
} }
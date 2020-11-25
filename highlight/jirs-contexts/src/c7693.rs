
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
      regex: Regex::new("-?%\\}"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288521949970548,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7711 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7719 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7722 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7717 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7716 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7720 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7714 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7712 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7715 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7713 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7721 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7724 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7726 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7709 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7718 })),
]
} }

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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\""),
      scope: vec![
        Scope {
            a: 47288629323956395,
            b: 5911081885106176,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1782 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1788 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1781 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1784 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1761 })),
]
} }
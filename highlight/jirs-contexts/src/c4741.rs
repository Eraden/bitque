
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
        a: 59955136424902700,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 59955136424902700,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 4772 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4758 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=-)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4742 }),
    ]),
      with_prototype: None
    }),
]
} }
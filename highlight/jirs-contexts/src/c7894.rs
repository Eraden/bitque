
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
        a: 46444883013992569,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444883013992569,
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
            a: 47288521965109419,
            b: 34058472181989376,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7958 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7971 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7998 })),
]
} }

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
        a: 844948916142080,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844948916142080,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 8069 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(package)\\b(?:\\s*([^ ;$]+)\\s*)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787168378880,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130665417034,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8075 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8080 })),
]
} }
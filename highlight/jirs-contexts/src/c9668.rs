
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9623 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9669 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9637 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9652 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))import(?=\\s*[\\(]\\s*[\\\"\\\'\\`]))"),
      scope: vec![
        Scope {
            a: 52636628121289143,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
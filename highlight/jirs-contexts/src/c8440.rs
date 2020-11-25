
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
      regex: Regex::new("(?=([\\])};,]|\\b(else|then)\\b))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8530 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8523 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8532 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8521 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8510 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8497 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8498 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\bor\\b|\\.|==|!=|!|\\<\\=|\\<|\\>\\=|\\>|&&|\\|\\||-\\>|//|\\?|\\+\\+|-|\\*|/(?=([^*]|(?m:$)))|\\+)"),
      scope: vec![
        Scope {
            a: 52636628107132928,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8502 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8499 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8528 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8527 })),
]
} }
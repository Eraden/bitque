
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
      regex: Regex::new("\\b(builtins|true|false|null)\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59955110645596160,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8437 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(scopedImport|import|isNull|abort|throw|baseNameOf|dirOf|removeAttrs|map|toString|derivationStrict|derivation)\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 61925255093944320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8438 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[0-9]+\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59955089170759680,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8439 }),
    ]),
      with_prototype: None
    }),
]
} }
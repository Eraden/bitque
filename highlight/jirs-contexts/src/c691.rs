
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
        a: 46443865079349248,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46443865079349248,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 783 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 824 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)\\)"),
      scope: vec![
        Scope {
            a: 46443865079349248,
            b: 0,
        },
        Scope {
            a: 47288521944400043,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
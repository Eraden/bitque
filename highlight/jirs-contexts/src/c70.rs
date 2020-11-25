
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
        a: 844442110001152,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844442110001152,
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
      regex: Regex::new("\\b(?i:End)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:Option\\s+Explicit)"),
      scope: vec![
        Scope {
            a: 52635837824761856,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 45 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 77 })),
]
} }

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
    Pattern::Include(ContextReference::Direct(ContextId { index: 55 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:End)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 22 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Case\\s+Else)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![
        Scope {
            a: 52636636701196292,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Case)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![
        Scope {
            a: 52636636701196292,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 52 }),
        ContextReference::Direct(ContextId { index: 50 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 77 })),
]
} }

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
        a: 845030520520704,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845030520520704,
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
      regex: Regex::new("\\{|\\}|;"),
      scope: vec![
        Scope {
            a: 50103314662883328,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9109 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9145 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9151 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9130 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9120 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9119 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9103 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9088 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(,)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620722028544,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }

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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3793 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3765 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3764 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(virtual|private|protected|public)\\b"),
      scope: vec![
        Scope {
            a: 48414439027245056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3581 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3703 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620724781056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\{|;)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
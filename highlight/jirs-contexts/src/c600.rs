
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 559 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 620 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 619 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(?:<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>)?\\s*::\\s*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 473 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(?:<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>)?\\s*::\\s*~\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(\\(|(?m:$)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 474 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(?:<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>)?::\\s*(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 475 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 669 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 601 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("\\b(namespace)\\s+(?=((?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)?(?!\\s*[;,]))"),
      scope: vec![
        Scope {
            a: 46444977478696960,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636692348928,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3652 }),
    ]),
      with_prototype: None
    }),
]
} }
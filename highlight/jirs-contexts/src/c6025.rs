
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
  prototype: Some(
    ContextId {
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\:)(?:function|data|export)\\b"),
      scope: vec![
        Scope {
            a: 48414576502112341,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:default|internal|hidden|protected|proc|data)\\b"),
      scope: vec![
        Scope {
            a: 48414439063158869,
            b: 24488322973827072,
        },
        Scope {
            a: 49258876878192725,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6121 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6068 })),
]
} }
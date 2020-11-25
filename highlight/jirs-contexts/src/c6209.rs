
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
        index: 6241,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6214 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6202 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6240 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\belse\\b"),
      scope: vec![
        Scope {
            a: 50103314682151177,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\belseif\\b"),
      scope: vec![
        Scope {
            a: 50103314682151320,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
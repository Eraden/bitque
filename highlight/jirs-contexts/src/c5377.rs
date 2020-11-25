
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
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5410 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5412 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5406 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5404 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5402 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5444 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|"),
      scope: vec![
        Scope {
            a: 52636628114800714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5310 }),
    ]),
      with_prototype: None
    }),
]
} }
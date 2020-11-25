
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5034 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5020 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(pub)\\b(?:(\\()(crate)(\\)))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629401026742,
            b: 20266198323167232,
        },
    ]),(3, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629401026731,
            b: 20266198323167232,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b)(?=\\s*:)"),
      scope: vec![
        Scope {
            a: 49259087306883144,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5009 }),
    ]),
      with_prototype: None
    }),
]
} }
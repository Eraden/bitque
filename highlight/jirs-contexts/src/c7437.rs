
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
      regex: Regex::new("\\b(sections)\\b"),
      scope: vec![
        Scope {
            a: 52636297533390848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(section)\\b"),
      scope: vec![
        Scope {
            a: 52636297533390848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b(end)\\b \\s+ \\b(sections)\\b  (\\s+ nowait \\b)? )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636297533390848,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636297533390848,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636297533390848,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7424 })),
]
} }
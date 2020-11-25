
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
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288629318582454,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9014 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620722028544,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9159 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9126 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9095 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9094 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9096 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9097 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9128 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9117 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9115 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9123 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9149 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9140 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9139 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9141 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9146 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[^\\s\'\"\\(\\)]+\\b"),
      scope: vec![
        Scope {
            a: 55451949097418752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
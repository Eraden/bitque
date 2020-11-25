
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7063 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\^[\\p{L}0-9\'._]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450153,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(and|when|or)\\b"),
      scope: vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7058 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(static member|member)\\s*([\\p{L}0-9\'`<>^._]+|``[\\p{L}0-9\' <>^._]+``)\\s*(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258571895930880,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7070 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7071 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7084 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7073 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7076 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7087 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7072 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7064 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7076 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7065 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7087 })),
]
} }
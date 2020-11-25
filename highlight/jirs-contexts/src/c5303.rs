
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
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<!\\[CDATA\\["),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 22517998136852480,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5198 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("</(?:([\\p{L}_][\\p{L}\\p{N}_.-]*)(:))?([\\p{L}_][\\p{L}\\p{N}_.-]*)>?"),
      scope: vec![
        Scope {
            a: 50103314746179664,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)((?:([\\p{L}_][\\p{L}\\p{N}_.-]*)(:))?([\\p{L}_][\\p{L}\\p{N}_.-]*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5306 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<\\?\\s*xml(?:\\s.*[>$]|\\b)"),
      scope: vec![
        Scope {
            a: 50103314746245200,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<\\?)\\s*((?:([\\p{L}_][\\p{L}\\p{N}_.-]*)(:))?([\\p{L}_][\\p{L}\\p{N}_.-]*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5306 }),
    ]),
      with_prototype: None
    }),
]
} }
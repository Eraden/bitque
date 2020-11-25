
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7064 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:)\\s*(\\()\\s*(static member|member)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7028 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7029 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:)\\s*(\\^[\\p{L}0-9\'._]+)\\s*(when)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450153,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7030 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:)\\s*([?\\p{L}0-9\'`^._ ]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
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
      regex: Regex::new("(->)\\s*(\\()?\\s*([?\\p{L}0-9\'`^._ ]+)*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(3, vec![
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
      regex: Regex::new("(\\*)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7031 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*)(\\s*([?\\p{L}0-9\'`^._ ]+))*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
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
      regex: Regex::new("(<(?![[:space:]]*\\)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7032 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7033 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7072 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7089 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7076 })),
]
} }
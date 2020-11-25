
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6720 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b(void|bool|num|int|double|dynamic|var|String|List|Map)\\b)\\s+(\\w+)(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576511352928,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6693 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(new)\\b\\s+(\\w+)(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638212947968000,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6694 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b(\\w+)\\b)\\s+(\\w+)(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 57711587231793152,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6695 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\w+)(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6696 }),
    ]),
      with_prototype: None
    }),
]
} }
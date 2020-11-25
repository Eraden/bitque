
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
      regex: Regex::new("^\\s*(?:\\b(void|bool|num|int|double|dynamic|var|String|List|Map)\\b)\\s+(get)\\s+(\\w+)\\s+(?==>)"),
      scope: vec![
        Scope {
            a: 46445763465642080,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576511352928,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638212947968000,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?:\\b(\\w+)\\b\\s+)?(get)\\s+(\\w+)\\s+(?==>)"),
      scope: vec![
        Scope {
            a: 46445763465642080,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 57711587231793152,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638212947968000,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(set)\\s+(\\w+)(?=\\()"),
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
        ContextReference::Direct(ContextId { index: 6697 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?:\\b(void|bool|num|int|double|dynamic|var|String|List|Map)\\b)\\s+(\\w+)(?=\\()"),
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
        ContextReference::Direct(ContextId { index: 6698 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?:\\b(return)\\b)\\s+(\\w+)(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636694970368,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49821981417144320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6699 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\b(new)\\b\\s+(\\w+)(?=\\()"),
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
        ContextReference::Direct(ContextId { index: 6700 }),
    ]),
      with_prototype: None
    }),
]
} }
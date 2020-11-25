
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46444243035881472,
        b: 0,
    },
    Scope {
        a: 46444243048988676,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444243035881472,
        b: 0,
    },
    Scope {
        a: 46444243048988676,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:End)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 18 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 71 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:Public(?:\\s+Default)?|Private)\\s+)?((?i:Property))(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439023837184,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474128388,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 19 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 80 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 34 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S"),
      scope: vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
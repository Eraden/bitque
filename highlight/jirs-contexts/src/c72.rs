
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
        a: 46444084122091520,
        b: 0,
    },
    Scope {
        a: 46444084132773892,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444084122091520,
        b: 0,
    },
    Scope {
        a: 46444084132773892,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 34 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 54 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Application_OnEnd|Application_OnStart|OnTransactionAbort|OnTransactionCommit|Session_OnEnd|Session_OnStart)\\b|\\b(?i:Class_Initialize|Class_Terminate)\\b"),
      scope: vec![
        Scope {
            a: 59392130630615044,
            b: 0,
        },
        Scope {
            a: 61925255101481204,
            b: 1125899906842624,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 32 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-zA-Z]\\w*|\\[(?:(?!%>|\\]).)*(?:\\]|(\\n|(?=%>)))"),
      scope: vec![
        Scope {
            a: 59392130630615044,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 32 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 65 }),
    ]),
      with_prototype: None
    }),
]
} }
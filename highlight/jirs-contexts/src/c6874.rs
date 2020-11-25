
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
  clear_scopes: Some(
    ClearAmount::All,
),
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%>"),
      scope: vec![
        Scope {
            a: 281496458035200,
            b: 0,
        },
        Scope {
            a: 52636787154550883,
            b: 0,
        },
        Scope {
            a: 47288521949642923,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<%[=/|]?"),
      scope: vec![
        Scope {
            a: 281496458035200,
            b: 0,
        },
        Scope {
            a: 52636787154550883,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6875 }),
        ContextReference::Direct(ContextId { index: 6858 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 6876 }),
    )
    }),
]
} }
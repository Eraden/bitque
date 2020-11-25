
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
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?{1,2})(?x:\n  MODULE|FUNCTION_NAME|FUNCTION_ARITY|MODULE_STRING|\n  FILE|LINE|MACHINE|OTP_RELEASE\n)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110666764308,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629340602388,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?{1,2}"),
      scope: vec![
        Scope {
            a: 59955136436568084,
            b: 0,
        },
        Scope {
            a: 47288629340602388,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1531 }),
    ]),
      with_prototype: None
    }),
]
} }
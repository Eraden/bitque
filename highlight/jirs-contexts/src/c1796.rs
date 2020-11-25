
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
        index: 1792,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\")(\\!)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451420828565525,
            b: 7036874417766400,
        },
        Scope {
            a: 47288629323956406,
            b: 5911081885106176,
        },
    ]),(2, vec![
        Scope {
            a: 52636636717449290,
            b: 5911081885106176,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1771 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 1772 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\!"),
      scope: vec![
        Scope {
            a: 52636636717449290,
            b: 5911081885106176,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1773 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 1774 }),
    )
    }),
]
} }

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
        index: 399,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?)?((\\[)(,*)(\\]))?(?:\\s*(\\*))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576488022027,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444990360649728,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ]),(4, vec![
        Scope {
            a: 47288620721831936,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288521961570475,
            b: 3096224743817216,
        },
    ]),(6, vec![
        Scope {
            a: 52636628123516939,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 390 }),
    ]),
      with_prototype: None
    }),
]
} }
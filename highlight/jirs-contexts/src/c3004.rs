
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
        index: 3019,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(else|fi|ftrue|if(case|cat|dim|eof|false|hbox|hmode|inner|mmode|num|odd|undefined|vbox|vmode|void|x)?))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636691038208,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032676,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:input))\\b"),
      scope: vec![
        Scope {
            a: 46444131426304036,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636748513316,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032676,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
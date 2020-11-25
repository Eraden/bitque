
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
      regex: Regex::new("(\\()((\\?=)|(\\?!)|(\\?<=)|(\\?<!))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 47288629318582995,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 46446476473729068,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46446476473794604,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 46446476473860140,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 46446476473925676,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2717 }),
    ]),
      with_prototype: None
    }),
]
} }
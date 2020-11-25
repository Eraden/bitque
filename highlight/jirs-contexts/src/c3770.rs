
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
      regex: Regex::new("((@)(interface|protocol))(?!.+;)\\s+([\\p{L}_][\\p{L}\\p{N}_]*)\\s*((:)(?:\\s*)([\\p{L}][\\p{L}\\p{N}]*))?(\\s|\\n)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466198528,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322318029,
            b: 15762598695796736,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450104,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629324873953,
            b: 72339309532807168,
        },
    ]),(7, vec![
        Scope {
            a: 59392186470432824,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 46448138574626816,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3656 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(implementation))\\s+([\\p{L}_][\\p{L}\\p{N}_]*)\\s*(?::\\s*([\\p{L}][\\p{L}\\p{N}]*))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466198528,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322318029,
            b: 15762598695796736,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450104,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392186470432824,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3657 }),
    ]),
      with_prototype: None
    }),
]
} }
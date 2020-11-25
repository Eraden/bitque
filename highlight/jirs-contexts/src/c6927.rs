
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
      regex: Regex::new("({)(\\d+)(,)?(\\d*)(})[?+]?"),
      scope: vec![
        Scope {
            a: 46447008997441635,
            b: 0,
        },
        Scope {
            a: 52636628154253356,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629366554806,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 59955089217882257,
            b: 12385324177031168,
        },
    ]),(3, vec![
        Scope {
            a: 47288620776620076,
            b: 27866022694354944,
        },
    ]),(4, vec![
        Scope {
            a: 59955089217882258,
            b: 12385324177031168,
        },
    ]),(5, vec![
        Scope {
            a: 47288629366554795,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[?*+][?+]?"),
      scope: vec![
        Scope {
            a: 46447008997441635,
            b: 0,
        },
        Scope {
            a: 52636628154253356,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("\\?((\\()(\\)))"),
      scope: vec![
        Scope {
            a: 52636636711616556,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314676580396,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 47288629333983414,
            b: 12385324177031168,
        },
    ]),(3, vec![
        Scope {
            a: 47288629333983403,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6921 }),
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?(\\()"),
      scope: vec![
        Scope {
            a: 52636636711616556,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629333983414,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6892 }),
    ]),
      with_prototype: None
    }),
]
} }
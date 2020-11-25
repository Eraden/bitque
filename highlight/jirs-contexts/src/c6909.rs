
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
      regex: Regex::new("\\[:.*?:\\]"),
      scope: vec![
        Scope {
            a: 50103314741329964,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[)(\\^?)(]?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636704866348,
            b: 27866022694354944,
        },
        Scope {
            a: 47288629327233206,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 52636628153794604,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 46444359002620003,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6889 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("(\\})([^[;\\s\\}\\]\\\\]]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521951477931,
            b: 21955048183431168,
        },
    ]),(2, vec![
        Scope {
            a: 50103314658754560,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5752 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(#)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711032873038,
            b: 0,
        },
        Scope {
            a: 47288629323038798,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5726 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5753 })),
]
} }
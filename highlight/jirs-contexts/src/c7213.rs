
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
      regex: Regex::new("(\\})|(\\).*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445029161042026,
            b: 0,
        },
        Scope {
            a: 47288521962160299,
            b: 29836347531329536,
        },
    ]),(2, vec![
        Scope {
            a: 50103314665963626,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\,"),
      scope: vec![
        Scope {
            a: 46445029161042026,
            b: 0,
        },
        Scope {
            a: 47288521962160320,
            b: 29836347531329536,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+"),
      scope: vec![
        Scope {
            a: 46445029161043263,
            b: 29836347531329536,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?!\\s)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7214 }),
        ContextReference::Direct(ContextId { index: 7215 }),
        ContextReference::Direct(ContextId { index: 7216 }),
    ]),
      with_prototype: None
    }),
]
} }

#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 281565172858880,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281565172858880,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 1822,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(commit)\\s+(\\h{7,}))?\\s*\\n"),
      scope: vec![
        Scope {
            a: 46445845059797015,
            b: 0,
        },
        Scope {
            a: 114280120459395093,
            b: 7881299347898368,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787014041621,
            b: 7881299347898368,
        },
    ]),(2, vec![
        Scope {
            a: 59955136432635925,
            b: 7881299347898368,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1816 }),
        ContextReference::Direct(ContextId { index: 1820 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 1817 }),
    )
    }),
]
} }
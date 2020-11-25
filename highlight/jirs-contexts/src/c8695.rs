
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
        a: 46444131394715782,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444131394715782,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![],
      captures: Some(vec![]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\$+)[a-zA-Z_\\x{7f}-\\x{ff}][a-zA-Z0-9_\\x{7f}-\\x{ff}]*)\\s*(?=,|\\))"),
      scope: vec![
        Scope {
            a: 46444131390196116,
            b: 37717646879227904,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087300788224,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514566,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\$+)[a-zA-Z_\\x{7f}-\\x{ff}][a-zA-Z0-9_\\x{7f}-\\x{ff}]*)(?:\\s*(=)\\s*)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087300788224,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514566,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111130758,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8696 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("(\\()?(\\b(?!null|false|true)[\\p{L}][\\p{L}\\p{N}_-]*\\b)(\\))?\\s*(\\=[^\\=|\\>])\\s*"),
      scope: vec![
        Scope {
            a: 49260513230913536,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521948004534,
            b: 41939771529887744,
        },
    ]),(2, vec![
        Scope {
            a: 49259087310291093,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521948004523,
            b: 41939771529887744,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130773,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
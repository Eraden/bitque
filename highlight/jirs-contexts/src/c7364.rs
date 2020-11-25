
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
      regex: Regex::new("(?xi:^  \\s*  (namelist)  \\s*  (/)([A-Za-z_][A-Za-z_0-9]*)(/) )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 651333993903751168,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52645759346343936,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47297739083808768,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130770667717,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47297739083808768,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7363 }),
    ]),
      with_prototype: None
    }),
]
} }
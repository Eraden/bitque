
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
        a: 46448104238284857,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46448104238284857,
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
      regex: Regex::new("((@)end)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466264064,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322318029,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3956 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3933 })),
]
} }
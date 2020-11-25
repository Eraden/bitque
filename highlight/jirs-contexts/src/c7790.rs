
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
        a: 844485072453675,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844485072453675,
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
      regex: Regex::new("`"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956666,
            b: 48132405701115904,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::ByScope { scope: Scope {
        a: 844609614643200,
        b: 0,
    }, sub_context: None  }),
]
} }
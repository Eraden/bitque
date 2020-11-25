
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
        a: 59392130632450433,
        b: 711005791171117056,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 59392130632450433,
        b: 711005791171117056,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((}))\\s*|(?=\\*/)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450433,
            b: 711005791171117056,
        },
    ]),(2, vec![
        Scope {
            a: 47288629367997388,
            b: 48143070104911872,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9571 })),
]
} }
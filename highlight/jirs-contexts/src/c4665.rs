
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
      regex: Regex::new("^\\s*((\\#+)\\s*(.+?)\\s*(?:-{4,}|={4,}|#{4,})[ \\t]*(?m:$)\\n?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711032873024,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323038784,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630090816,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
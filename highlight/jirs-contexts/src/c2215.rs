
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
      regex: Regex::new("(<jsp:scriptlet>|<jsp:expression>|<jsp:declaration>)|(<%(?!--)[!=]?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230165332007,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521949642934,
            b: 10977524091715584,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2218 })),
]
} }
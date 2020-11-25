
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
      regex: Regex::new("^(?i:(from))\\s+[^\\s:@]+(?:[:@](\\S+))?(?:\\s+(?i:(as))\\s+(\\S+))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636695035904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130642806885,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636695035904,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49267346514116608,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6734 }),
    ]),
      with_prototype: None
    }),
]
} }
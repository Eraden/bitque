
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
      regex: Regex::new("(=\\?)(?:(utf-8|windows-1251|iso-[a-z0-9-]+\\b)(\\?))?([qQ])(\\?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 29273397577908224,
        },
    ]),(2, vec![
        Scope {
            a: 59392130642804840,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59955110644023296,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6971 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=\\?)(utf-8|windows-1251|iso-[a-z0-9-]+\\b)(\\?)([bB])(\\?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 29273397577908224,
        },
    ]),(2, vec![
        Scope {
            a: 59392130642804840,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59955110644023296,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6972 }),
    ]),
      with_prototype: None
    }),
]
} }
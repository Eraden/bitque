
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
        a: 46445269533589504,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445269533589504,
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
      regex: Regex::new(">"),
      scope: vec![
        Scope {
            a: 46445269533589504,
            b: 0,
        },
        Scope {
            a: 47288521955803307,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.{3}"),
      scope: vec![
        Scope {
            a: 52636628128366604,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(typename|struct|union|enum\\s+class|enum\\s+struct|enum|class)\\b"),
      scope: vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 660 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 661 })),
]
} }
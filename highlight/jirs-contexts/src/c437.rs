
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
        a: 46444243036405760,
        b: 0,
    },
    Scope {
        a: 46444328935751680,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444243036405760,
        b: 0,
    },
    Scope {
        a: 46444328935751680,
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
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 46444243036405760,
            b: 0,
        },
        Scope {
            a: 46444328935751680,
            b: 0,
        },
        Scope {
            a: 47288521951477931,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 562 })),
]
} }
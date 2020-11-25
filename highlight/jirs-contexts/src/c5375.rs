
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
        a: 46444874421633649,
        b: 21392416057589760,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444874421633649,
        b: 21392416057589760,
    },
],
  meta_include_prototype: true,
  clear_scopes: Some(
    ClearAmount::TopN(1),
),
  prototype: Some(
    ContextId {
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";;&?|;&"),
      scope: vec![
        Scope {
            a: 46444874421633649,
            b: 21392416057589760,
        },
        Scope {
            a: 47288689466409585,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5378 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5430 })),
]
} }

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
  prototype: Some(
    ContextId {
        index: 3019,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\(\\)]"),
      scope: vec![
        Scope {
            a: 59955200887226404,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\[\\]]"),
      scope: vec![
        Scope {
            a: 59955200888471588,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\)[\\{\\}]"),
      scope: vec![
        Scope {
            a: 59955200860094705,
            b: 10133099161583616,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629362033076,
            b: 67835624006090752,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
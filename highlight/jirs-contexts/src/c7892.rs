
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
      regex: Regex::new("@doc"),
      scope: vec![
        Scope {
            a: 49260057962545152,
            b: 0,
        },
        Scope {
            a: 61925255093485568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(raw)?(\"\"\"|\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439225,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323038902,
            b: 34058472181989376,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7893 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("#"),
      scope: vec![
        Scope {
            a: 47288629323038904,
            b: 89509390486339584,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5844 }),
    ]),
      with_prototype: None
    }),
]
} }
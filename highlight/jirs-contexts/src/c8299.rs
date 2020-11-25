
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
      regex: Regex::new("(@)({)([a-zA-Z0-9_-][\\w-]*)(})"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087300067328,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514555,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 34621422135410688,
        },
    ]),(3, vec![
        Scope {
            a: 55452159557959680,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
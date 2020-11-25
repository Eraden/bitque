
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
      regex: Regex::new("(\\$)([a-zA-Zx7f-xff\\$]|::)([a-zA-Z0-9_x7f-xff\\$]|::)*\\b"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 37717646879227904,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514566,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\{)(?:[a-zA-Zx7f-xff\\$]|::)(?:[a-zA-Z0-9_x7f-xff\\$]|::)*(\\})"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 37717646879227904,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514566,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514566,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
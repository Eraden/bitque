
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
      regex: Regex::new("^(\\s*(\\.{1,5}))\\s+(?=\\S)"),
      scope: vec![
        Scope {
            a: 114280017426908715,
            b: 23643898043695104,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451949114459691,
            b: 23643898043695104,
        },
    ]),(2, vec![
        Scope {
            a: 59955089180327968,
            b: 23643898043695104,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
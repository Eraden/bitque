
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
      regex: Regex::new("^(\\[\\[)([^\\[].*)(\\]\\])\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444230253609044,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629413937334,
            b: 23643898043695104,
        },
    ]),(2, vec![
        Scope {
            a: 114280588699107841,
            b: 23643898043695104,
        },
    ]),(3, vec![
        Scope {
            a: 47288629413937323,
            b: 23643898043695104,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
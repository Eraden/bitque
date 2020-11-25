
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
      regex: Regex::new("(\\[)([^-!]\\S*)(\\])"),
      scope: vec![
        Scope {
            a: 46444990361305110,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629335752886,
            b: 5911069000204288,
        },
    ]),(2, vec![
        Scope {
            a: 55451949097877526,
            b: 0,
        },
        Scope {
            a: 59392130648571925,
            b: 6192449487634432,
        },
    ]),(3, vec![
        Scope {
            a: 47288629335752875,
            b: 5911069000204288,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1695 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("(?xi:^  \\s*  (\\d+)  \\s+  (format)  \\s*  (\\()  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130768963781,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52641172321271808,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 646839138682667008,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7354 }),
    ]),
      with_prototype: None
    }),
]
} }
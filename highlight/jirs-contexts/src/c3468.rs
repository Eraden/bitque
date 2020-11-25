
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
      regex: Regex::new("(?:(:)\\s*([^=]+))?(?:(=)|(=)\\s*(?=fun(?:ction)\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620732711938,
            b: 14636698788954112,
        },
    ]),(2, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628102152192,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628102152192,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3521 })),
]
} }
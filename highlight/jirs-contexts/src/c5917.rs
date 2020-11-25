
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46444328946958991,
        b: 23643898043695104,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444328946958991,
        b: 23643898043695104,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("^\\1\\s*(?m:$)\\n?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038891,
            b: 23643898043695104,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5977 })),
]
} }
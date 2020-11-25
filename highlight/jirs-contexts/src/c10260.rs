
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
        a: 46445780657963169,
        b: 0,
    },
    Scope {
        a: 845262453604352,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445780657963169,
        b: 0,
    },
    Scope {
        a: 845262453604352,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("none"),
      scope: vec![
        Scope {
            a: 59955110653395105,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5464 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10294 })),
]
} }
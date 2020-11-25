
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
        a: 46444157140271104,
        b: 0,
    },
    Scope {
        a: 51510878542233661,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444157140271104,
        b: 0,
    },
    Scope {
        a: 51510878542233661,
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
      regex: Regex::new("^=cut\\b"),
      scope: vec![
        Scope {
            a: 46444157140271104,
            b: 0,
        },
        Scope {
            a: 51510878542233661,
            b: 0,
        },
        Scope {
            a: 59392130632123610,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4278 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4281 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4280 })),
]
} }
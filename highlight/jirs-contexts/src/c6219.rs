
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
  prototype: Some(
    ContextId {
        index: 6241,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6200 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bIN\\b"),
      scope: vec![
        Scope {
            a: 49258876865808059,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bLISTS\\b"),
      scope: vec![
        Scope {
            a: 49258876865808060,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bITEMS\\b"),
      scope: vec![
        Scope {
            a: 49258876865808061,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bRANGE\\b"),
      scope: vec![
        Scope {
            a: 49258876865808062,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
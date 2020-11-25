
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
        a: 46444466403475468,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444466403475468,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::ByScope { scope: Scope {
        a: 844476469739520,
        b: 0,
    }, sub_context: Some("preprocessor-line-continuation".to_string())  }),
    Pattern::Include(ContextReference::ByScope { scope: Scope {
        a: 844476469739520,
        b: 0,
    }, sub_context: Some("preprocessor-line-ending".to_string())  }),
    Pattern::Include(ContextReference::ByScope { scope: Scope {
        a: 844476469739520,
        b: 0,
    }, sub_context: Some("preprocessor-comments".to_string())  }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![
        Scope {
            a: 59392130630484056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
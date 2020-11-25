
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
        a: 46444084122091520,
        b: 0,
    },
    Scope {
        a: 48414576474128555,
        b: 1125899906842624,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444084122091520,
        b: 0,
    },
    Scope {
        a: 48414576474128555,
        b: 1125899906842624,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 34 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Function|Sub|Property)(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![
        Scope {
            a: 46444084122091520,
            b: 0,
        },
        Scope {
            a: 48414576474128555,
            b: 1125899906842624,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 79 })),
]
} }
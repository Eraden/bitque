
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
      regex: Regex::new("\\b((?i:Public(?:\\s+Default)?|Private)\\s+)?((?i:Sub|Function))(?=\\s|(?m:$)|:|\'|\\b(?i:REM)\\b|%>)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444084122091520,
            b: 0,
        },
        Scope {
            a: 46444084132773892,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 48414439023837184,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474128388,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 66 }),
        ContextReference::Direct(ContextId { index: 73 }),
    ]),
      with_prototype: None
    }),
]
} }
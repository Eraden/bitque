
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
      regex: Regex::new("\\b(func)\\s+(\\B\\$[0-9]+|\\b[\\w^\\d][\\w\\d]*\\b|\\B`[\\w^\\d][\\w\\d]*`\\B|[/=\\-+!*%<>&|\\^~.]+)\\s*(?=\\(|<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474128530,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392100565844114,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9238 }),
    ]),
      with_prototype: None
    }),
]
} }
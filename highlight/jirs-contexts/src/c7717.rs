
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
      regex: Regex::new("(?<=[\\s\\(\\[\\{:,])(attribute|block|constant|cycle|date|divisible by|dump|include|max|min|parent|random|range|same as|source|template_from_string)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255093157888,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629327560886,
            b: 32651097298436096,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7688 }),
    ]),
      with_prototype: None
    }),
]
} }
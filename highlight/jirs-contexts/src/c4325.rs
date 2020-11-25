
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
      regex: Regex::new("/"),
      scope: vec![
        Scope {
            a: 47288521955803318,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4230 }),
        ContextReference::Direct(ContextId { index: 4769 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4231 }),
    )
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4288 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4289 })),
]
} }
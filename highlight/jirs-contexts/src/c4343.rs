
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
      regex: Regex::new("(s|tr|y)([^\\w\\s\\)\\]\\}\\>])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255089553408,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521955803318,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4239 }),
        ContextReference::Direct(ContextId { index: 4769 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4240 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([^\\w\\s\\)\\]\\}\\>])"),
      scope: vec![
        Scope {
            a: 47288620756172861,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4241 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4298 })),
]
} }
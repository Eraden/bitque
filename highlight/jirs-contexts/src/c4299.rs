
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
      regex: Regex::new("\\brequire\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636717450469,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4348 }),
        ContextReference::Direct(ContextId { index: 4347 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\buse\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636717450339,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4207 }),
        ContextReference::Direct(ContextId { index: 4291 }),
        ContextReference::Direct(ContextId { index: 4330 }),
        ContextReference::Direct(ContextId { index: 4329 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bno\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636717450460,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4208 }),
        ContextReference::Direct(ContextId { index: 4291 }),
        ContextReference::Direct(ContextId { index: 4330 }),
        ContextReference::Direct(ContextId { index: 4329 }),
    ]),
      with_prototype: None
    }),
]
} }
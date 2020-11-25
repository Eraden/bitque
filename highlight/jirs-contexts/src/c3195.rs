
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
      regex: Regex::new("((?:bash|sh|zsh)\\s+-c\\s+)([\'\"`])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444217268961280,
            b: 0,
        },
        Scope {
            a: 55451949099646976,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217268961280,
            b: 0,
        },
        Scope {
            a: 46445132096995328,
            b: 0,
        },
        Scope {
            a: 47288521963733174,
            b: 13510798882111488,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3156 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3157 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3196 }),
    ]),
      with_prototype: None
    }),
]
} }
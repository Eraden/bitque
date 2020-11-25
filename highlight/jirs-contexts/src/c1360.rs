
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=!|~|\\+|\\-|\\*|&|\\bcast\\b|\\bdelete\\b|\\bnew\\b|\\bimport\\b|\\bmixin\\b|\\bis\\b|\\b__traits\\b|\\bfunction\\b|\\bdelegate\\b|[0-9]|\\[|\\(|(?=`|[rxq]?\"|q{)|\\b(null|true|false|__FILE__|__FILE_FULL_PATH__|__MODULE__|__LINE__|__FUNCTION__|__PRETTY_FUNCTION__|__DATE__|__EOF__|__TIME__|__TIMESTAMP__|__VENDOR__|__VERSION__|__ctfe)\\b|\\b(this|super)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1282 }),
        ContextReference::Direct(ContextId { index: 1357 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=([\\p{L}_][\\p{L}0-9_]*|\\]|\\))(\\s+[\\p{L}_][\\p{L}0-9_]*)|\\b(const|immutable|inout|shared)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1256 }),
        ContextReference::Direct(ContextId { index: 1335 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1282 }),
        ContextReference::Direct(ContextId { index: 1232 }),
    ]),
      with_prototype: None
    }),
]
} }
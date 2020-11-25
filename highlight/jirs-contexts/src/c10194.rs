
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
      regex: Regex::new("(?i)(?=jade(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'jade\\\'|\"jade\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10127 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=pug(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'pug\\\'|\"pug\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10133 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=slm(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'slm\\\'|\"slm\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10137 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10195 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
]
} }
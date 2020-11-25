
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
      regex: Regex::new("(?i)(?=sass(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'sass\\\'|\"sass\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10084 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=scss(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'scss\\\'|\"scss\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10102 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=stylus(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'stylus\\\'|\"stylus\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10106 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=postcss\\?parser=sugarss(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'postcss\\?parser=sugarss\\\'|\"postcss\\?parser=sugarss\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10088 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=postcss(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'postcss\\\'|\"postcss\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10092 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=less(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\\\'less\\\'|\"less\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10097 }),
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
        ContextReference::Direct(ContextId { index: 10163 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("(^\\s*)(?=class\\s+(([.\\p{L}\\p{N}_:]+ControllerTest(\\s*<\\s*[.\\p{L}\\p{N}_:]+)?)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4720 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)(?=class\\s+(([.\\p{L}\\p{N}_:]+Controller\\b(\\s*<\\s*[.\\p{L}\\p{N}_:]+)?)|(<<\\s*[.\\p{L}\\p{N}_:]+)))(?!.+\\bend\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4721 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)(?=module\\s+((([\\p{Lu}]\\w*::)*)[\\p{Lu}]\\w*)Helper\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4722 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)(?=class\\s+(([.\\p{L}\\p{N}_:]+(\\s*<\\s*ActionMailer::Base\\b))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4723 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)(?=class\\s+.+ActiveRecord::Base\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4724 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)(?=class\\s+.+ActiveRecord::Migration\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4725 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)(?=class\\s+(?![.\\p{L}\\p{N}_:]+ControllerTest)(([.\\p{L}\\p{N}_:]+Test(\\s*<\\s*[.\\p{L}\\p{N}_:]+)?)|(<<\\s*[.\\p{L}\\p{N}_:]+)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4728 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^\\s*)ActionController::Routing::Routes"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4729 }),
    ]),
      with_prototype: None
    }),
]
} }
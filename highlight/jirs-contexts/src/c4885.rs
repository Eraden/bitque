
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
      regex: Regex::new("<<[-~][\"`]?((?:[\\p{Lu}_]_)?(?:JS|JAVASCRIPT))[\"`]?"),
      scope: vec![
        Scope {
            a: 55451949170753602,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4886 }),
        ContextReference::Direct(ContextId { index: 4954 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<[-~]\'((?:[\\p{Lu}_]_)?(?:JS|JAVASCRIPT))\'"),
      scope: vec![
        Scope {
            a: 55451949170753602,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4887 }),
        ContextReference::Direct(ContextId { index: 4954 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<[\"`]?((?:[\\p{Lu}_]_)?(?:JS|JAVASCRIPT))[\"`]?"),
      scope: vec![
        Scope {
            a: 55451949170753602,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4888 }),
        ContextReference::Direct(ContextId { index: 4954 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<\'((?:[\\p{Lu}_]_)?(?:JS|JAVASCRIPT))\'"),
      scope: vec![
        Scope {
            a: 55451949170753602,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4889 }),
        ContextReference::Direct(ContextId { index: 4954 }),
    ]),
      with_prototype: None
    }),
]
} }
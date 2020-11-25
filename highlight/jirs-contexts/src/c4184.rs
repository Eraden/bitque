
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
  clear_scopes: Some(
    ClearAmount::TopN(1),
),
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^=end\\b"),
      scope: vec![
        Scope {
            a: 59392130632123610,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcss\\b"),
      scope: vec![
        Scope {
            a: 59955136470515726,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4185 }),
        ContextReference::Direct(ContextId { index: 1017 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4189 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bhtml\\b"),
      scope: vec![
        Scope {
            a: 59955136470515717,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4190 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4191 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:js|javascript)\\b"),
      scope: vec![
        Scope {
            a: 59955136470515755,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4192 }),
        ContextReference::Direct(ContextId { index: 7848 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4193 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bjson\\b"),
      scope: vec![
        Scope {
            a: 59955136470515750,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4194 }),
        ContextReference::Direct(ContextId { index: 2200 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4195 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bsql\\b"),
      scope: vec![
        Scope {
            a: 59955136470515781,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4196 }),
        ContextReference::Direct(ContextId { index: 5102 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4186 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bxml\\b"),
      scope: vec![
        Scope {
            a: 59955136470515792,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4187 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4188 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^=\\b[_\\p{L}]\\w*\\b"),
      scope: vec![
        Scope {
            a: 50103314735104061,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4288 })),
]
} }
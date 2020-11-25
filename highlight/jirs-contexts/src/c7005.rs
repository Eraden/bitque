
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46445780657963584,
        b: 29273397577908224,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445780657963584,
        b: 29273397577908224,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=\\S|(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7003 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6991 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([a-zA-Z0-9-]+)\\b(=)(?!\\s|=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087310291048,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130728,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6974 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7020 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7016 })),
]
} }
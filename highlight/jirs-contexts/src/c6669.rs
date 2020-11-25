
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
      regex: Regex::new("#\\{(\\})"),
      scope: vec![
        Scope {
            a: 844828669837315,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288521949642846,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 844828669837315,
            b: 206321157928910848,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288521949642846,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6605 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#@)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291329,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#@@)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291147,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#\\$)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("(@)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291329,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514498,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@@)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291147,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514498,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514498,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(!|@|&|`|\'|\\+|\\d+|~|=|/|\\\\|,|;|\\.|<|>|_|\\*|\\$|\\?|:|\"|-[0adFiIlpv])"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 385058051608018944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514498,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ENV)\\["),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087305965634,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4855 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(::)?(\\b[\\p{Lu}]\\w*)(?=((\\.|::)[\\p{L}_]|\\[))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788251050050,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925366759030784,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{Lu}]\\w*\\b"),
      scope: vec![
        Scope {
            a: 49259087305965634,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
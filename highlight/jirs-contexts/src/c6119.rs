
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%)(\\[))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466403475541,
            b: 24488322973827072,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ]),(3, vec![
        Scope {
            a: 46444990365499479,
            b: 0,
        },
        Scope {
            a: 47288521961570486,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6041 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%)\\+)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636628115456085,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%)(\\?\\??))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636628115456085,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ]),(3, vec![
        Scope {
            a: 49259061538914389,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%\\$+)((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?:(\\:)?|\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632975938,
            b: 23925746682560512,
        },
    ]),(4, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%)(\\!))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636628115456085,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ]),(3, vec![
        Scope {
            a: 47288629322514517,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6042 }),
    ]),
      with_prototype: None
    }),
]
} }
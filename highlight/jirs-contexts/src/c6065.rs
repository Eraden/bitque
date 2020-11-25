
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
      regex: Regex::new("(\\.\\.@)((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?:(\\:)?|\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
        Scope {
            a: 48414439029145687,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632975938,
            b: 23925746682560512,
        },
    ]),(3, vec![
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
      regex: Regex::new("(?:(\\.)?|\\b)((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?:(\\:)?|\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
        Scope {
            a: 48414439029145687,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632974421,
            b: 24488322973827072,
        },
    ]),(3, vec![
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
      regex: Regex::new("(\\.)([0-9]+(?:[\\p{L}\\p{N}_$#@~.?]*))(?:(\\:)?|\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
        Scope {
            a: 48414439029145687,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632974421,
            b: 24488322973827072,
        },
    ]),(3, vec![
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
      regex: Regex::new("(?:(\\.)?|\\b)([0-9$@~](?:[\\p{L}\\p{N}_$#@~.?]*))(?:(\\:)?|\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
        Scope {
            a: 48414439029145687,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667471060,
            b: 59954535117291520,
        },
    ]),(3, vec![
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
      regex: Regex::new("((%)(\\d+))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314664456447,
            b: 23925746682560512,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514687,
            b: 23925746682560512,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(3, vec![
        Scope {
            a: 49259087308718165,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((%%)((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?:(\\:)?|\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314664456447,
            b: 23925746682560512,
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
]
} }
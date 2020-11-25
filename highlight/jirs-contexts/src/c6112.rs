
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
    Pattern::Include(ContextReference::Named("preprocessor-comments".to_string())),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?:\\s+(?:(\\d+(?:(-)(?:\\d+|\\*))?)(\\+)?(\\.nolist\\b)?))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615295,
            b: 23925746682560512,
        },
    ]),(2, vec![
        Scope {
            a: 49258876855320661,
            b: 24488322973827072,
        },
    ]),(3, vec![
        Scope {
            a: 52636628115456273,
            b: 23925746682560512,
        },
    ]),(4, vec![
        Scope {
            a: 48414439040288316,
            b: 23925746682560512,
        },
    ]),(5, vec![
        Scope {
            a: 48414439040288270,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6103 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
]
} }

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
      regex: Regex::new("(?<!\\.)\\b(injectGlobal|keyframes)\\s*(`)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881188233259,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956666,
            b: 51228630444933120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7789 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(styled)(\\.)([_$a-zA-Z][$\\w]*)\\s*(`)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087346401323,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788227653632,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258881188233259,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956666,
            b: 51228630444933120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7790 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(styled)\\s*(?=(\\((?>(?>[^()]+)|\\g<-1>)*\\))\\s*`)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444883125010475,
            b: 0,
        },
        Scope {
            a: 49258881136394240,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7791 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("(@@[_$a-zA-Z][$\\w]*|static|return)(?=\\s*[<(])|(?=\\s*<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628252885035,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7733 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=[]\"\'])\\s*(?=[<(])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7734 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((?>get|set)\\s+)\n(?>\n  ((\')((?>[^\'\\\\]|\\\\.)*)(\'))|\n  ((\")((?>[^\"\\\\]|\\\\.)*)(\"))|\n  (([_$a-zA-Z][$\\w]*|\\d+))\n)(?=\\s*[<(])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576465346560,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451420818341888,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956406,
            b: 12103423998558208,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 12103423998558208,
        },
    ]),(6, vec![
        Scope {
            a: 55451420818341888,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629323956406,
            b: 12103423998558208,
        },
    ]),(8, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 47288629323956395,
            b: 12103423998558208,
        },
    ]),(10, vec![
        Scope {
            a: 55451949099319296,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7735 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?>\n  ((\')((?>[^\'\\\\]|\\\\.)*)(\'))|\n  ((\")((?>[^\"\\\\]|\\\\.)*)(\"))|\n  (([_$a-zA-Z][$\\w]*|\\d+))\n)(?=\\s*[<(])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451420818341888,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 12103423998558208,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 12103423998558208,
        },
    ]),(5, vec![
        Scope {
            a: 55451420818341888,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629323956406,
            b: 12103423998558208,
        },
    ]),(7, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 47288629323956395,
            b: 12103423998558208,
        },
    ]),(9, vec![
        Scope {
            a: 55451949099319296,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7736 }),
    ]),
      with_prototype: None
    }),
]
} }
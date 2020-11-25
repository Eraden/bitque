
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
      regex: Regex::new("(?x)\n(?>\n  ((\')((?>[^\'\\\\]|\\\\.)*)(\'))|\n  ((\")((?>[^\"\\\\]|\\\\.)*)(\"))|\n  (([_$a-zA-Z][$\\w]*|\\d+))\n)\n\\s*(:)\n\\s*(?:\\b(async)\\s+)?\n\\s*(?=(\\((?>(?>[^()]+)|\\g<-1>)*\\))\\s*(=>))"),
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
    ]),(11, vec![
        Scope {
            a: 47288620737429547,
            b: 0,
        },
    ]),(12, vec![
        Scope {
            a: 48414576465346560,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7761 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?>\n  ((\')((?>[^\'\\\\]|\\\\.)*)(\'))|\n  ((\")((?>[^\"\\\\]|\\\\.)*)(\"))|\n  (([_$a-zA-Z][$\\w]*|\\d+))\n)\n\\s*(:)\n\\s*(?:\\b(async)\\s+)?\n\\s*\\b([_$a-zA-Z][$\\w]*)\\s*(=>)"),
      scope: vec![
        Scope {
            a: 46444131368960424,
            b: 12103423998558208,
        },
    ],
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
    ]),(11, vec![
        Scope {
            a: 47288620737429547,
            b: 0,
        },
    ]),(12, vec![
        Scope {
            a: 48414576465346560,
            b: 0,
        },
    ]),(13, vec![
        Scope {
            a: 49258876850208811,
            b: 0,
        },
    ]),(14, vec![
        Scope {
            a: 48414576474128808,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
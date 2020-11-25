
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
      regex: Regex::new("(?x)\n(?:([_$a-zA-Z][$\\w]*)\\s*(=)\\s*)?\n(?:(async)\\s+)?\n(function)(?>\\s*(\\*)|(?=[\\s(<]))\n\\s*([_$a-zA-Z][$\\w]*)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576465346560,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576474128427,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52638522221068331,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7773 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\\b_?[A-Z][$\\w]*)?\n(\\.)(prototype)\n(\\.)([_$a-zA-Z][$\\w]*)\n\\s*(=)\n\\s*(?:(async)\\s+)?\n\\s*(function)(?>\\s*(\\*)|(?=[\\s(<]))\n\\s*([_$a-zA-Z][$\\w]*)?\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632319019,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259061576269867,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 48414576465346560,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 48414576474128427,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 52638522221068331,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\\b_?[A-Z][$\\w]*)?\n(\\.)([_$a-zA-Z][$\\w]*)\n\\s*(=)\n\\s*(?:(async)\\s+)?\n\\s*(function)(?>\\s*(\\*)|(?=[\\s(<]))\n\\s*([_$a-zA-Z][$\\w]*)?\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632319019,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576465346560,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 48414576474128427,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 52638522221068331,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 59392130630615083,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7775 }),
    ]),
      with_prototype: None
    }),
]
} }
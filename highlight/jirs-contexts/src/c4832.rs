
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
        a: 46444131370795008,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444131370795008,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(self)(\\.)(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(?:[?!]|=(?![>=]))?|===?|>[>=]?|<=>|<[<=]?|[%&`/\\|]|\\*\\*?|=?~|[-+]@?|\\[\\]=?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259061526528000,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226932802,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615106,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4925 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("===?|>[>=]?|<=>|<[<=]?|[%&`/\\|]|\\*\\*?|=?~|[-+]@?|\\[\\]=?"),
      scope: vec![
        Scope {
            a: 59392130630615106,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4925 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?:(::)|(\\.)))?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(?:[?!]|=(?![>=]))?"),
      scope: vec![
        Scope {
            a: 59392130630615106,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925461268496450,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788251050050,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788226932802,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4925 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
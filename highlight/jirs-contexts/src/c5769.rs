
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
        a: 281496456724480,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281496456724480,
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
      regex: Regex::new("(^h[1-6]([<>=()]+)?)(\\([^)]*\\)|\\{[^}]*\\})*(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123019,
            b: 22236523160141824,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632123019,
            b: 22236523160141824,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5761 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^bq([<>=()]+)?)(\\([^)]*\\)|\\{[^}]*\\})*(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123333,
            b: 22236523160141824,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632123333,
            b: 22236523160141824,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5762 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^fn[0-9]+([<>=()]+)?)(\\([^)]*\\)|\\{[^}]*\\})*(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123239,
            b: 22236523160141824,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632123239,
            b: 22236523160141824,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5763 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^table([<>=()]+)?)(\\([^)]*\\)|\\{[^}]*\\})*(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123239,
            b: 22236523160141824,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632123239,
            b: 22236523160141824,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5764 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5765 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2107 })),
]
} }
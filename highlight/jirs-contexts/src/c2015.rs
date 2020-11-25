
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
      regex: Regex::new("(?x)^\\s*\n(?:(?:\\b(?:(public|private|protected)|(static)|(final)|(native|synchronized|abstract|threadsafe|transient))\\b\\s*)*) # modifier\n(class)\\s+\n(\\w+)\\s* # identifier"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439060275233,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439055228961,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414439071547425,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414439038320673,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576475832353,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 59392130632450251,
            b: 9288674231451648,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1986 }),
    ]),
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("^(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^p([<>=()]+)?)(\\([^)]*\\)|\\{[^}]*\\})*(\\.)"),
      scope: vec![
        Scope {
            a: 59392130630091692,
            b: 22236523160141824,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632123308,
            b: 22236523160141824,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632123308,
            b: 22236523160141824,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5768 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2107 })),
]
} }
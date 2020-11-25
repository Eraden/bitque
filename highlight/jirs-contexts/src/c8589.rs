
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
      regex: Regex::new("(\\[)\\s*\\b(?i)(cmdletbinding|alias|outputtype|parameter|validatenotnull|validatenotnullorempty|validatecount|validateset|allownull|allowemptycollection|allowemptystring|validatescript|validaterange|validatepattern|validatelength|supportswildcards)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521993814198,
            b: 36873221949095936,
        },
    ]),(2, vec![
        Scope {
            a: 61925255114915971,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8568 }),
    ]),
      with_prototype: None
    }),
]
} }
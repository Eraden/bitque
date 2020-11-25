
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
      regex: Regex::new("(</)((?i:script))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122658,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(//).*?((?=</script)|(?m:$)\\n?)"),
      scope: vec![
        Scope {
            a: 51510711028613163,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038763,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/\\*"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038763,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7661 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7701 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7723 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7725 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7710 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7848 })),
]
} }
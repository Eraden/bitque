
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
        a: 845034815488000,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845034815488000,
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
      regex: Regex::new("(?x)\n(?: ^ [ \\t]* | [ \\t]+ )\n(?:(\\#) \\p{Print}* )?\n(\\n|\\z)\n"),
      scope: vec![
        Scope {
            a: 51510711032873041,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038904,
            b: 89509390486339584,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7874 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5881 })),
]
} }
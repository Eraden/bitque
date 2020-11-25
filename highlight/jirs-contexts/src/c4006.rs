
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
      regex: Regex::new("(?=::\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4007 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(::)\n(?:\n    (class)\\b\n    |\n    ((\\$+)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\n    |\n    (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\n)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788251050042,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955041921531904,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087305310266,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59955136420315194,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4098 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(self|static|parent)\\b"),
      scope: vec![
        Scope {
            a: 49259061526003712,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4105 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4156 })),
]
} }
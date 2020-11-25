
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
      regex: Regex::new("^(?i:(?:\\s?|#)+(\\.)(COMPONENT|DESCRIPTION|EXAMPLE|EXTERNALHELP|FORWARDHELPCATEGORY|FORWARDHELPTARGETNAME|FUNCTIONALITY|INPUTS|LINK|NOTES|OUTPUTS|REMOTEHELPRUNSPACE|ROLE|SYNOPSIS))"),
      scope: vec![
        Scope {
            a: 51511655917617283,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955016174403715,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628125220995,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\s?(\\.)(PARAMETER|FORWARDHELPTARGETNAME|FORWARDHELPCATEGORY|REMOTEHELPRUNSPACE|EXTERNALHELP)\\s+([a-z0-9-_]+))"),
      scope: vec![
        Scope {
            a: 51511655917617283,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955016174403715,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628125220995,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628125220995,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }

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
  prototype: Some(
    ContextId {
        index: 1792,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:no-?)?(?:ul|strike|reverse|italic|dim|bold|blink)\\b"),
      scope: vec![
        Scope {
            a: 61925409749008405,
            b: 7036874417766400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:yellow|white|red|normal|magenta|green|cyan|blue|black|auto)\\b"),
      scope: vec![
        Scope {
            a: 61925409739046933,
            b: 7036874417766400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:25[0-5]|2[0-4][0-9]|1\\d\\d|[1-9][0-9]|[0-9])\\b"),
      scope: vec![
        Scope {
            a: 59955136441680405,
            b: 5911081885106176,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
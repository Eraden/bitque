
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
      regex: Regex::new("(?x)\n# optional closing keyword\n(?i:\n  ( (?:close|fixe|resolve)[ds]? | fix )     # keyword\n  \\s* (:)? \\s*             # optional colon separator\n)?                         # keyword is optional\n# user/repo#issue\n(\n  (?:\n    \\b(?: \\w+ (/) )? \\w+   # user (optional) / repo\n  )?\n  (\\#)[0-9]+               # issue number\n)\\b"),
      scope: vec![
        Scope {
            a: 46444118525870101,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787056902165,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757877339,
            b: 190558649427427328,
        },
    ]),(3, vec![
        Scope {
            a: 59955136418415268,
            b: 5910974510923776,
        },
    ]),(4, vec![
        Scope {
            a: 47288620732514980,
            b: 5910974510923776,
        },
    ]),(5, vec![
        Scope {
            a: 47288629322449572,
            b: 5910974510923776,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
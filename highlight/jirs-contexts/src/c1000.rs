
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
      regex: Regex::new("(--)((?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+)"),
      scope: vec![
        Scope {
            a: 61925375377932302,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629344337934,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925375377932500,
            b: 3940649673949184,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("\\s*(?x:\n    (:{1,2})(?:before|after|first-line|first-letter) # CSS1 & CSS2 require : or ::\n  | (::)(-(?:moz|ms|webkit)-)?(?:(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)) # CSS3 requires ::\n)\\b"),
      scope: vec![
        Scope {
            a: 59392186489110542,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375377735694,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
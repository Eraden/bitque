
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
      regex: Regex::new("(?x)\\b__(?:\n  # generic object\n  class|dict|doc|module|name|\n  # module-specific / global\n  all|file|package|\n  # functions & methods\n  annotations|closure|code|defaults|func|globals|kwdefaults|self|qualname|\n  # classes (attributes)\n  bases|prepare|slots|metaclass|mro|\n  # Python 2\n  members|methods\n)__\\b"),
      scope: vec![
        Scope {
            a: 61925246511546430,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
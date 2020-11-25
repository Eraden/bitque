
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
        index: 3111,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?!(?x:(?:\n  and|break|do|elseif|else|end|false|for|function|goto|if|in|\n  local|nil|not|or|repeat|return|then|true|until|while\n)(?!(?:[A-Za-z0-9_]))))(?:(?:[A-Za-z_])(?:[A-Za-z0-9_])*))(?=\\s*=\\s*function(?!(?:[A-Za-z0-9_])))"),
      scope: vec![
        Scope {
            a: 59392130630615087,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?!(?x:(?:\n  and|break|do|elseif|else|end|false|for|function|goto|if|in|\n  local|nil|not|or|repeat|return|then|true|until|while\n)(?!(?:[A-Za-z0-9_]))))(?:(?:[A-Za-z_])(?:[A-Za-z0-9_])*))(?=\\s*(?:\\(|\"|\'|\\[=*\\[|\\{))"),
      scope: vec![
        Scope {
            a: 49258881136656384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?!(?x:(?:\n  and|break|do|elseif|else|end|false|for|function|goto|if|in|\n  local|nil|not|or|repeat|return|then|true|until|while\n)(?!(?:[A-Za-z0-9_]))))(?:(?:[A-Za-z_])(?:[A-Za-z0-9_])*))"),
      scope: vec![
        Scope {
            a: 49259087295086592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }
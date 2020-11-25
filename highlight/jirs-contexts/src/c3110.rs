
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
      regex: Regex::new("(?x:__(?:\n  # special\n  index|newindex|call|tostring|len|i?pairs|gc\n  # math operators\n  |unm|add|sub|mul|i?div|mod|pow|concat\n  # bitwise operators\n  |band|bor|bxor|bnot|shl|shr\n  # comparison\n  |eq|lt|le\n)(?!(?:[A-Za-z0-9_])))(?=\\s*=\\s*function(?!(?:[A-Za-z0-9_])))"),
      scope: vec![
        Scope {
            a: 46444371887718400,
            b: 0,
        },
        Scope {
            a: 59392130630615087,
            b: 0,
        },
        Scope {
            a: 61925255145848879,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?!(?x:(?:\n  and|break|do|elseif|else|end|false|for|function|goto|if|in|\n  local|nil|not|or|repeat|return|then|true|until|while\n)(?!(?:[A-Za-z0-9_]))))(?:(?:[A-Za-z_])(?:[A-Za-z0-9_])*))(?=\\s*=\\s*function(?!(?:[A-Za-z0-9_])))"),
      scope: vec![
        Scope {
            a: 46444371887718400,
            b: 0,
        },
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
            a: 46444371887718400,
            b: 0,
        },
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
      regex: Regex::new("(?:__(?:metatable|mode)(?!(?:[A-Za-z0-9_])))"),
      scope: vec![
        Scope {
            a: 46444371887718400,
            b: 0,
        },
        Scope {
            a: 61925461304344623,
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
            a: 46444371887718400,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3113 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3086 })),
]
} }

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
      regex: Regex::new("%%"),
      scope: vec![
        Scope {
            a: 59955200847315607,
            b: 5910974510923776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%x\\d\\d"),
      scope: vec![
        Scope {
            a: 59955200875954839,
            b: 5910974510923776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n  %[-+ ]?\n  (?:\n    [HhTtPpdDesSfbBNmn]| # single-char\n    [ac][nNeEdDrtiI]|    # author and committer\n    G[G?DKFP]|           # GPG\n    g[dnNeEs]            # reflog\n  )\n"),
      scope: vec![
        Scope {
            a: 59955136434012823,
            b: 5910974510923776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1762 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1765 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1763 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1764 })),
]
} }
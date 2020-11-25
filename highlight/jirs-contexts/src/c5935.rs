
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46450071322689620,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46450071322689620,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?<=\\S)(\\*)       # delimiter star that must be preceded by a nonspace character\n(?!\\w)            # ...and followed by a nonword character"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629354365179,
            b: 48132581794775040,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5964 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5981 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5963 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5980 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5992 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5991 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5960 })),
]
} }
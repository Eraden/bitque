
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
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=\\s*([\\p{Lu}_][\\p{Lu}\\p{Nd}_]*|(?!(?:boolean|byte|char|short|int|float|long|double)|(?:public|private|protected|static|final|native|synchronized|strictfp|abstract|transient|default|volatile))[\\p{Ll}_][\\p{L}\\p{N}_]*)\\s*[,;{(])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2239 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2308 })),
]
} }
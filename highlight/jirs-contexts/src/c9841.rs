
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
      regex: Regex::new("((?=;|}|(\\s+(of|in)\\s+)|^\\s*(?m:$)|;|^\\s*abstract\\b|^\\s*async\\b|^\\s*class\\b|^\\s*const\\b|^\\s*declare\\b|^\\s*enum\\b|^\\s*export\\b|^\\s*function\\b|^\\s*import\\b|^\\s*interface\\b|^\\s*let\\b|^\\s*module\\b|^\\s*namespace\\b|^\\s*return\\b|^\\s*type\\b|^\\s*var\\b)|((?<=\\S)(?<!^let|[^\\._$\\p{L}\\p{N}]let|^var|[^\\._$\\p{L}\\p{N}]var)(?=\\s*(?m:$))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9881 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9998 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10000 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(,)\\s*(?!\\S)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620757155991,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9842 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9958 })),
]
} }
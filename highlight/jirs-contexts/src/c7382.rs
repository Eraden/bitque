
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7303 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7308 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7309 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7311 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7313 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7318 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7319 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7320 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7321 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7322 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7326 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7325 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7328 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7329 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7330 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7331 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7332 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7333 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7334 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7335 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7337 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7345 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7348 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7351 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7352 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7353 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7357 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7364 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7365 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7367 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7376 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7387 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7378 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7380 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7324 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7375 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7388 })),
]
} }
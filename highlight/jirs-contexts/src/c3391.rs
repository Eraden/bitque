
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3402 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3405 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3406 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3407 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3410 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3409 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3400 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3450 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3403 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3404 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3401 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3408 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3413 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3412 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3411 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3414 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3415 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3416 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3417 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3418 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3419 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3420 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3421 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3422 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3423 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3424 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3426 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3425 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3427 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3428 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3429 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3430 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3431 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3432 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3433 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3434 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3435 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3437 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3436 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3438 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3439 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3440 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3441 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3442 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3443 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3444 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3445 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3446 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3447 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3448 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3449 })),
]
} }
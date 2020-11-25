
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7849 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7843 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7830 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7804 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7854 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7855 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7827 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7822 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7841 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7832 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7836 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7838 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7824 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7808 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7839 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7846 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7844 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7834 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7835 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7825 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7837 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7828 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7801 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7797 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7840 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7847 })),
]
} }
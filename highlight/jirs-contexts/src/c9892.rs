
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9914 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9968 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9962 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9973 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9898 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9870 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9859 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9953 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9867 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9975 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9934 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9911 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9944 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9890 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9896 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9929 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9971 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9952 })),
]
} }
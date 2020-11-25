
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1897 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1898 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1899 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1900 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1901 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1902 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1903 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1904 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1905 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1906 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1907 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1908 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1909 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1910 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1911 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1912 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1913 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1914 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1915 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1916 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1917 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1918 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1919 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1920 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1921 })),
]
} }
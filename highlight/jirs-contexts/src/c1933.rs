
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
      regex: Regex::new("(?=\\b(?!\\b(?:break|case|chan|const|continue|default|defer|else|fallthrough|for|func|go|goto|if|import|interface|map|package|range|return|select|struct|switch|type|var)\\b)[\\p{L}_][\\p{L}\\p{N}_]*\\b(?:(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*,(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\b(?!\\b(?:break|case|chan|const|continue|default|defer|else|fallthrough|for|func|go|goto|if|import|interface|map|package|range|return|select|struct|switch|type|var)\\b)[\\p{L}_][\\p{L}\\p{N}_]*\\b)*(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*:=)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1859 }),
    ]),
      with_prototype: None
    }),
]
} }
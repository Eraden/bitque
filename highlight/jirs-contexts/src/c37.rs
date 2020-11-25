
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 34 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 54 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-zA-Z]\\w*|\\[(?:(?!%>|\\]).)*(?:\\]|(\\n|(?=%>)))"),
      scope: vec![
        Scope {
            a: 59392130632318980,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 56 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 79 })),
]
} }

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
        a: 281565172924416,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281565172924416,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1740 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<(?=\\S+?>)"),
      scope: vec![
        Scope {
            a: 47288629322449564,
            b: 51228535955652608,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1745 }),
        ContextReference::Direct(ContextId { index: 1746 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^ \\t<#][^<#\\n]+?(?=[ \\t]*(<|(?m:$)))"),
      scope: vec![
        Scope {
            a: 46444118526787605,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
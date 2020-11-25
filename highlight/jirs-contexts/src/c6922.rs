
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
      regex: Regex::new("(\\(\\*)((?>NO_START_OPT|UTF8?|UCP|CRLF|CR|LF|ANYCRLF|ANY|BSR_ANYCRLF|BSR_UNICODE|LIMIT_(?>MATCH|RECURSION)=))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629337129142,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 48414439111723151,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6898 }),
    ]),
      with_prototype: None
    }),
]
} }
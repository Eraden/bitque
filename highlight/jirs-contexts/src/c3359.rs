
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
        a: 46447408444473393,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46447408444473393,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=(?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3278 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3370 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)(?!(?:[ ]{,3}(?=\\d+\\.|[*+-])\\d*([*+.-])\\s))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3279 }),
    ]),
      with_prototype: None
    }),
]
} }
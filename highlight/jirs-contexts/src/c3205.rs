
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
      regex: Regex::new("(?x)^\n(?=  \\s*(?m:$)\n|    (?x:\n    [ ]{,3}                          # between 0 to 3 spaces\n    (?:                              # followed by one of the following:\n            [-](?:[ ]{,2}[-]){2,}    # - a dash,        followed by the following at least twice: between 0 to 2 spaces followed by a dash\n        |   [*](?:[ ]{,2}[*]){2,}    # - a star,        followed by the following at least twice: between 0 to 2 spaces followed by a star\n        |   [_](?:[ ]{,2}[_]){2,}    # - an underscore, followed by the following at least twice: between 0 to 2 spaces followed by an underscore\n    )\n    [ \\t]*(?m:$)                          # followed by any number of tabs or spaces, followed by the end of the line\n)\n|    (?:[ ]{,3}>(?:.|(?m:$)))\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?=  (?:[ ]{,3}>(?:.|(?m:$)))\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3206 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?=  (?:[ ]{4}|\\t)\n|    (?:[#]{1,6}\\s*)\n|    (?x:\n    [ ]{,3}                          # between 0 to 3 spaces\n    (?:                              # followed by one of the following:\n            [-](?:[ ]{,2}[-]){2,}    # - a dash,        followed by the following at least twice: between 0 to 2 spaces followed by a dash\n        |   [*](?:[ ]{,2}[*]){2,}    # - a star,        followed by the following at least twice: between 0 to 2 spaces followed by a star\n        |   [_](?:[ ]{,2}[_]){2,}    # - an underscore, followed by the following at least twice: between 0 to 2 spaces followed by an underscore\n    )\n    [ \\t]*(?m:$)                          # followed by any number of tabs or spaces, followed by the end of the line\n)\n|    (?:[ ]{,3}(?=\\d+\\.|[*+-])\\d*([*+.-])\\s)\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3207 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3210 }),
    ]),
      with_prototype: None
    }),
]
} }
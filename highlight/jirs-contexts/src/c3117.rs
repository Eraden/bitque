
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
  prototype: Some(
    ContextId {
        index: 3111,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:_G|_VERSION)(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 61925409709424687,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  assert|collectgarbage|dofile|error|getmetatable|ipairs|load|loadfile\n  |next|pairs|pcall|print|rawequal|rawget|rawlen|rawset|select\n  |setmetatable|tonumber|tostring|type|xpcall\n  |require|getfenv|module|setfenv|unpack\n)(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 61925255090602031,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3084 })),
]
} }

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
      regex: Regex::new("\\b(wait|videoinput|triggerinfo|triggerconfig|trigger|stoppreview|stop|start|set|save|propinfo|preview|peekdata|obj2mfile|load|isvalid|isrunning|islogging|imaqtool|imaqreset|imaqmontage|imaqmem|imaqhwinfo|imaqhelp|imaqfind|getsnapshot|getselectedsource|getdata|get|flushdata|disp|delete|closepreview|clear)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044139,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
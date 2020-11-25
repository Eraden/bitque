
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
      regex: Regex::new("\\b(writeasync|write|wait|trend|stop|start|showopcevents|set|serveritems|serveritemprops|save|removepublicgroup|refresh|readasync|read|propinfo|peekdata|openosf|opctool|opcsupport|opcstruct2timeseries|opcstruct2array|opcserverinfo|opcreset|opcregister|opcread|opcqstr|opcqparts|opcqid|opchelp|opcfind|opcda|opccallback|obj2mfile|makepublic|load|isvalid|getnamespace|getdata|get|genslwrite|genslread|flushdata|flatnamespace|disp|disconnect|delete|copyobj|connect|clonegroup|cleareventlog|cancelasync|additem|addgroup)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044145,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
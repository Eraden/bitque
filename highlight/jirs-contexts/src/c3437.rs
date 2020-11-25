
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
      regex: Regex::new("\\b(zpk|trim|tf|ss|size|sim|setoutdist|setname|setmpcsignals|setmpcdata|setindist|setestim|set|qpdantz|plot|pack|mpcverbosity|mpcstate|mpcsimopt|mpcprops|mpcmove|mpchelp|mpc|getoutdist|getname|getmpcdata|getindist|getestim|get|d2d|compare|cloffset)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044143,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
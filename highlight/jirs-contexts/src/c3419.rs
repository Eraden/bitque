
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
      regex: Regex::new("\\b(wait|trigger|stop|start|softscope|size|showdaqevents|setverify|set|save|putvalue|putsample|putdata|propinfo|peekdata|obj2mfile|muxchanidx|makenames|load|length|isvalid|issending|isrunning|islogging|isdioline|ischannel|inspect|getvalue|getsample|getdata|get|flushdata|disp|digitalio|delete|dec2binvec|daqreset|daqregister|daqread|daqmem|daqhwinfo|daqhelp|daqfind|daqcallback|clear|binvec2dec|analogoutput|analoginput|addmuxchannel|addline|addchannel)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044126,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
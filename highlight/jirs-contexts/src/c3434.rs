
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
      regex: Regex::new("\\b(visa|update|udp|trigger|tmtool|tcpip|stopasync|spoll|size|set|serialbreak|serial|selftest|scanstr|save|resolvehost|remove|record|readasync|query|propinfo|obj2mfile|midtest|midedit|methods|memwrite|memunmap|memread|mempoke|mempeek|memmap|makemid|load|length|iviconfigurationstore|isvalid|invoke|instrreset|instrnotify|instrid|instrhwinfo|instrhelp|instrfindall|instrfind|instrcallback|inspect|icdevice|gpib|geterror|get|fwrite|fscanf|fread|fprintf|fopen|flushoutput|flushinput|fgets|fgetl|fclose|echoudp|echotcpip|disp|disconnect|devicereset|delete|connect|commit|clrdevice|clear|binblockwrite|binblockread|add)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044141,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
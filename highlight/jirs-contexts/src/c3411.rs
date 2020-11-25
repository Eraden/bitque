
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
      regex: Regex::new("\\b(actxcontrol|actxcontrollist|actxcontrolselect|actxGetRunningServer|actxserver|addproperty|calllib|callSoapService|createClassFromWsdl|createSoapMessage|ddeadv|ddeexec|ddeinit|ddepoke|ddereq|ddeterm|ddeunadv|deleteproperty|enableservice|eventlisteners|events|Execute|GetCharArray|GetFullMatrix|GetVariable|GetWorkspaceData|import|instrcallback|instrfind|instrfindall|interfaces|invoke|javaaddpath|javaArray|javachk|javaclasspath|javaMethod|javaObject|javarmpath|libfunctions|libfunctionsview|libisloaded|libpointer|libstruct|loadlibrary|MaximizeCommandWindow|MinimizeCommandWindow|move|parseSoapResponse|PutCharArray|PutFullMatrix|PutWorkspaceData|readasync|record|registerevent|release|send|serial|serialbreak|stopasync|unloadlibrary|unregisterallevents|unregisterevent|usejava)\\b"),
      scope: vec![
        Scope {
            a: 61926891471437824,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
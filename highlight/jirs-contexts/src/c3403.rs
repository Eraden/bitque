
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
      regex: Regex::new("\\b(addpath|assignin|builddocsearchdb|cd|checkin|checkout|clc|clear|clipboard|cmopts|commandhistory|commandwindow|computer|copyfile|customverctrl|dbclear|dbcont|dbdown|dbquit|dbstack|dbstatus|dbstep|dbstop|dbtype|dbup|debug|demo|diary|dir|doc|docopt|docsearch|dos|echodemo|edit|exit|fileattrib|filebrowser|finish|format|genpath|getenv|grabcode|help|helpbrowser|helpwin|home|hostid|info|keyboard|license|lookfor|ls|matlab|matlabrc|matlabroot|memory|mkdir|mlint|mlintrpt|more|movefile|notebook|openvar|pack|partialpath|path|path2rc|pathdef|pathsep|pathtool|perl|playshow|prefdir|preferences|profile|profsave|publish|pwd|quit|recycle|rehash|restoredefaultpath|rmdir|rmpath|savepath|setenv|startup|support|system|toolboxdir|type|undocheckout|unix|ver|verctrl|verLessThan|version|web|what|whatsnew|which|winqueryreg|workspace)\\b|(^\\s*!.*(?m:$))"),
      scope: vec![
        Scope {
            a: 52640029716185088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
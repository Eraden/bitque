
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
      regex: Regex::new("\\b(addpref|align|dialog|errordlg|export2wsdlg|getappdata|getpixelposition|getpref|ginput|guidata|guide|guihandles|helpdlg|inputdlg|inspect|listdlg|listfonts|menu|movegui|msgbox|openfig|printdlg|printpreview|questdlg|rmappdata|rmpref|selectmoveresize|setappdata|setpixelposition|setpref|textwrap|uibuttongroup|uicontextmenu|uicontrol|uigetdir|uigetfile|uigetpref|uimenu|uiopen|uipanel|uipushtool|uiputfile|uiresume|uisave|uisetcolor|uisetfont|uisetpref|uistack|uitoggletool|uitoolbar|uiwait|waitbar|waitfor|waitforbuttonpress|warndlg)\\b"),
      scope: vec![
        Scope {
            a: 61925255088898048,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
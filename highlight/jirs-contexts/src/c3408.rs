
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
      regex: Regex::new("\\b(addframe|ascii|audioplayer|audiorecorder|aufinfo|auread|auwrite|avifile|aviinfo|aviread|beep|binary|cdfepoch|cdfinfo|cdfread|cdfwrite|csvread|csvwrite|daqread|dlmread|dlmwrite|exifread|feof|ferror|fgetl|fgets|filehandle|filemarker|fileparts|filesep|fitsinfo|fitsread|fopen|fprintf|fread|frewind|fscanf|fseek|ftell|ftp|fullfile|fwrite|gunzip|gzip|hdf|hdf5|hdf5info|hdf5read|hdf5write|hdfinfo|hdfread|hdftool|imfinfo|importdata|imread|imwrite|lin2mu|load|memmapfile|mget|mmfileinfo|movie2avi|mput|mu2lin|multibandread|multibandwrite|open|rename|save|sendmail|sound|soundsc|tar|tempdir|tempname|textread|textscan|todatenum|uiimport|untar|unzip|urlread|urlwrite|wavfinfo|wavplay|wavread|wavrecord|wavwrite|winopen|wk1finfo|wk1read|wk1write|xlsfinfo|xlsread|xlswrite|xmlread|xmlwrite|xslt|zip)\\b"),
      scope: vec![
        Scope {
            a: 48414512041361408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
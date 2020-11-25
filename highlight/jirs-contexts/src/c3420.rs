
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
      regex: Regex::new("\\b(width|versioncolumns|update|unregister|tables|tableprivileges|supports|sql2native|setdbprefs|set|runstoredprocedure|rsmd|rows|rollback|resultset|register|querytimeout|querybuilder|procedures|procedurecolumns|primarykeys|ping|namecolumn|logintimeout|isurl|isreadonly|isnullcolumn|isjdbc|isdriver|isconnection|insert|indexinfo|importedkeys|getdatasources|get|fetchmulti|fetch|fastinsert|exportedkeys|exec|drivermanager|driver|dmd|database\\.fetch|database|cursor\\.fetch|crossreference|confds|commit|columns|columnprivileges|columnnames|cols|close|clearwarnings|bestrowid|attr)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044127,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
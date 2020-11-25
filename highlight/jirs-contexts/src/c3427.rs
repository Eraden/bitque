
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
      regex: Regex::new("\\b(zeroyield|zeroprice|tfutyieldbyrepo|tfutpricebyrepo|tfutimprepo|tfutbyyield|tfutbyprice|tbillyield2disc|tbillyield|tbillval01|tbillrepo|tbillprice|tbilldisc2yield|stepcpnyield|stepcpnprice|stepcpncfamounts|psaspeed2rate|psaspeed2default|mbsyield2speed|mbsyield2oas|mbsyield|mbswal|mbsprice2speed|mbsprice2oas|mbsprice|mbspassthrough|mbsoas2yield|mbsoas2price|mbsnoprepay|mbsdury|mbsdurp|mbsconvy|mbsconvp|mbscfamounts|liborprice|liborfloat2fixed|liborduration|convfactor|cfamounts|cdyield|cdprice|cdai|cbprice|bkput|bkfloorlet|bkcaplet|bkcall)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044134,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
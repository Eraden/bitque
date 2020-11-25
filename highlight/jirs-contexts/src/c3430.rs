
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
      regex: Regex::new("\\b(ret2price|price2ret|ppTSTest|ppARTest|ppARDTest|parcorr|lratiotest|lbqtest|lagmatrix|hpfilter|garchsim|garchset|garchpred|garchplot|garchma|garchinfer|garchget|garchfit|garchdisp|garchcount|garchar|dfTSTest|dfARTest|dfARDTest|crosscorr|autocorr|archtest|aicbic)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044137,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("\\b(tpaps|titanium|subplus|stmak|stcol|spterms|sprpp|spmak|splpp|splinetool|spcrv|spcol|spaps|spapi|spap2|sorted|slvblk|rsmak|rscvn|rpmak|ppmak|optknt|newknt|knt2mlt|knt2brk|getcurve|franke|fnzeros|fnxtr|fnval|fntlr|fnrfn|fnplt|fnmin|fnjmp|fnint|fndir|fnder|fncmb|fnchg|fnbrk|fn2fm|cscvn|csaps|csapi|csape|chbpnt|bspline|bspligui|brk2knt|bkbrk|aveknt|augknt|aptknt)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044150,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
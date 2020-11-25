
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
      regex: Regex::new("\\b(wvarchg|wtreemgr|wthrmngr|wthresh|wthcoef2|wthcoef|wtbxmngr|wtbo|wscalogram|write|wrev|wrcoef2|wrcoef|wpviewcf|wptree|wpthcoef|wpsplt|wprec2|wprec|wprcoef|wpjoin|wpfun|wpdencmp|wpdec2|wpdec|wpcutree|wpcoef|wpbmpen|wp2wtree|wnoisest|wnoise|wmulden|wmspca|wmaxlev|wkeep|wfusmat|wfusimg|wfilters|wfbmesti|wfbm|wextend|wentropy|wenergy2|wenergy|wdencmp|wden|wdcenergy|wdcbm2|wdcbm|wcodemat|wbmpen|waverec2|waverec|wavenames|wavemngr|wavemenu|waveinfo|wavefun2|wavefun|wavedemo|wavedec2|wavedec|wave2lp|upwlev2|upwlev|upcoef2|upcoef|treeord|treedpth|tnodes|thselect|symwavf|symaux|swt2|swt|shanwavf|set|scal2frq|readtree|read|rbiowavf|qmf|plot|pat2cwav|orthfilt|ntree|ntnode|noleaves|nodesplt|nodepar|nodejoin|nodedesc|nodeasc|mswthresh|mswden|mswcmptp|mswcmpscr|mswcmp|morlet|meyeraux|meyer|mexihat|mdwtrec|mdwtdec|mdwtcluster|lwtcoef2|lwtcoef|lwt2|lwt|lsinfo|ls2filt|liftwave|liftfilt|leaves|laurpoly|laurmat|iswt2|iswt|istnode|isnode|intwave|ind2depo|ilwt2|ilwt|idwt2|idwt|get|gauswavf|filt2ls|fbspwavf|entrupd|dyadup|dyaddown|dwtmode|dwt2|dwt|dtree|drawtree|displs|disp|detcoef2|detcoef|depo2ind|ddencmp|dbwavf|dbaux|cwt|coifwavf|cmorwavf|chgwdeccfs|cgauwavf|cfs2wpt|centfrq|bswfun|biorwavf|biorfilt|besttree|bestlevt|appcoef2|appcoef|allnodes|addlift)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044155,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
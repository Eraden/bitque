
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
      regex: Regex::new("\\b(wcsens|wcnorm|wcmargin|wcgopt|wcgain|usubs|uss|usimsamp|usiminfo|usimfill|usample|ureal|uplot|umat|ultidyn|ufrd|udyn|ucomplexm|ucomplex|sysic|symdec|stack|stabproj|squeeze|slowfast|skewdec|simplify|showlmi|setmvar|setlmis|sectf|sdlsim|sdhinfsyn|sdhinfnorm|schurmr|robuststab|robustperf|robopt|repmat|reduce|randuss|randumat|randatom|quadstab|quadperf|pvinfo|pvec|psys|psinfo|popov|polydec|pdsimul|pdlstab|normalized2actual|newlmi|ncfsyn|ncfmr|ncfmargin|mussvextract|mussv|msfsyn|modreal|mktito|mkfilter|mixsyn|mincx|matnbr|mat2dec|ltrsyn|ltiarray2uss|loopsyn|loopsens|loopmargin|lmivar|lmiterm|lmireg|lminbr|lmiinfo|lmiedit|lftdata|isuncertain|ispsys|imp2ss|imp2exp|icsignal|iconnect|icomplexify|hinfsyn|hinfgs|hankelsv|hankelmr|h2syn|h2hinfsyn|gridureal|gevp|getlmis|genphase|gapmetric|fitmagfrd|fitfrd|feasp|evallmi|drawmag|dmplot|dksyn|dkitopt|diag|delmvar|dellmi|defcx|decnbr|decinfo|decay|dec2mat|dcgainmr|cpmargin|complexify|cmsclsyn|bstmr|bilin|balancmr|augw|aff2pol|actual2normalized)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044148,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
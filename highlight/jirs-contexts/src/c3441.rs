
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
      regex: Regex::new("\\b(writeva|write|timeresp|smith|setop|semilogy|semilogx|rfmodel\\.rational|rfdata\\.power|rfdata\\.noise|rfdata\\.nf|rfdata\\.network|rfdata\\.mixerspur|rfdata\\.ip3|rfdata\\.data|rfckt\\.txline|rfckt\\.twowire|rfckt\\.shuntrlc|rfckt\\.seriesrlc|rfckt\\.series|rfckt\\.rlcgline|rfckt\\.passive|rfckt\\.parallelplate|rfckt\\.parallel|rfckt\\.mixer|rfckt\\.microstrip|rfckt\\.lclowpasstee|rfckt\\.lclowpasspi|rfckt\\.lchighpasstee|rfckt\\.lchighpasspi|rfckt\\.lcbandstoptee|rfckt\\.lcbandstoppi|rfckt\\.lcbandpasstee|rfckt\\.lcbandpasspi|rfckt\\.hybridg|rfckt\\.hybrid|rfckt\\.delay|rfckt\\.datafile|rfckt\\.cpw|rfckt\\.coaxial|rfckt\\.cascade|rfckt\\.amplifier|restore|read|polar|plotyy|plot|loglog|listparam|listformat|impulse|getz0|getop|freqresp|extract|circle|calculate|analyze)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044147,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
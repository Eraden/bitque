
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
      regex: Regex::new("\\b(zpkdata|zpk|zgrid|zero|totaldelay|tfdata|tf|stepplot|stepinfo|step|stack|stabsep|ssdata|ssbal|ss2ss|ss|sminreal|size|sisotool|sisoinit|sigmaplot|sigma|sgrid|setoptions|setdelaymodel|set|series|rss|rlocusplot|rlocus|reshape|reg|real|pzplot|pzmap|pole|place|parallel|pade|ord2|obsvf|obsv|nyquistplot|nyquist|norm|nicholsplot|nichols|ngrid|ndims|modsep|modred|minreal|margin|lyapchol|lyap|ltiview|ltiprops|ltimodels|lsimplot|lsiminfo|lsim|lqry|lqrd|lqr|lqgreg|lqg|lft|kalmd|kalman|issiso|isproper|isempty|isdt|isct|iopzplot|iopzmap|inv|interp|initialplot|initial|impulseplot|impulse|imag|hsvplot|hsvd|hasdelay|gram|getoptions|getdelaymodel|get|gensig|gdare|gcare|fselect|freqresp|frdata|frd|fnorm|filt|feedback|fcat|evalfr|estim|esort|dssdata|dss|dsort|drss|dlyapchol|dlyap|dlqr|delayss|delay2z|dcgain|dare|damp|d2d|d2c|ctrlpref|ctrbf|ctrb|covar|connect|conj|chgunits|care|canon|c2d|bodeplot|bodemag|bode|bandwidth|balred|balreal|augstate|append|allmargin|acker|abs)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044124,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
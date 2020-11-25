
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
      regex: Regex::new("\\b(zpkdata|zpk|wavenet|view|unitgain|treepartition|timestamp|tfdata|tf|struc|step|ssdata|ss|spafdr|spa|size|simsd|sim|sigmoidnet|setstruc|setpname|setpar|setinit|set|selstruc|segment|saturation|rplr|rpem|roe|resid|resample|realdata|rbj|rarx|rarmax|pzmap|pwlinear|present|predict|polyreg|polydata|poly1d|plot|pexcit|pem|pe|oe|nyquist|nuderst|noisecnv|nlhw|nlarx|nkshift|neuralnet|n4sid|misdata|midprefs|merge|lintan|linear|linapp|ivx|ivstruc|ivar|iv4|isreal|init|impulse|ifft|idss|idresamp|idproc|idpoly|idnlmodel|idnlhw|idnlgrey|idnlarx|idmodel|idmdlsim|idinput|idgrey|idfrd|idfilt|ident|iddata|idarx|getreg|getpar|getinit|getexp|get|fselect|freqresp|frd|fpe|fft|ffplot|feedback|fcat|evaluate|etfe|diff|detrend|delayest|deadzone|d2c|customreg|customnet|cra|covf|compare|c2d|bode|bj|balred|arxstruc|arxdata|arx|armax|ar|aic|advice|addreg|EstimationInfo)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044153,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
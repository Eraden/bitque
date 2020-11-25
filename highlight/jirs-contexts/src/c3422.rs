
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
      regex: Regex::new("\\b(zplane|zpkshiftc|zpkshift|zpkrateup|zpklp2xn|zpklp2xc|zpklp2mbc|zpklp2mb|zpklp2lp|zpklp2hp|zpklp2bsc|zpklp2bs|zpklp2bpc|zpklp2bp|zpkftransf|zpkbpc2bpc|zerophase|window|validstructures|tf2cl|tf2ca|stepz|specifyall|sos|setspecs|set2int|scaleopts|scalecheck|scale|reset|reorder|reffilter|realizemdl|qreport|polyphase|phasez|phasedelay|parallel|order|nstates|normalizefreq|normalize|norm|noisepsdopts|noisepsd|multistage|msesim|msepred|mfilt\\.linearinterp|mfilt\\.iirwdfinterp|mfilt\\.iirwdfdecim|mfilt\\.iirinterp|mfilt\\.iirdecim|mfilt\\.holdinterp|mfilt\\.firtdecim|mfilt\\.firsrc|mfilt\\.firinterp|mfilt\\.firfracinterp|mfilt\\.firfracdecim|mfilt\\.firdecim|mfilt\\.fftfirinterp|mfilt\\.farrowsrc|mfilt\\.cicinterp|mfilt\\.cicdecim|mfilt\\.cascade|mfilt|measure|maxstep|limitcycle|lagrange|kaiserwin|isstable|issos|isreal|isminphase|ismaxphase|islinphase|isfir|isallpass|int|info|impz|iirshiftc|iirshift|iirrateup|iirpowcomp|iirpeak|iirnotch|iirls|iirlpnormc|iirlpnorm|iirlp2xn|iirlp2xc|iirlp2mbc|iirlp2mb|iirlp2lp|iirlp2hp|iirlp2bsc|iirlp2bs|iirlp2bpc|iirlp2bp|iirlinphase|iirgrpdelay|iirftransf|iircomb|iirbpc2bpc|ifir|help|grpdelay|gain|freqz|freqsamp|freqrespopts|freqrespest|firtype|firpr2chfb|firnyquist|firminphase|firls|firlpnorm|firlp2lp|firlp2hp|firhalfband|firgr|fireqint|firceqrip|fircband|filtstates\\.cic|filterbuilder|filter|fftcoeffs|fdesign\\.rsrc|fdesign\\.peak|fdesign\\.parameq|fdesign\\.octave|fdesign\\.nyquist|fdesign\\.notch|fdesign\\.lowpass|fdesign\\.isinclp|fdesign\\.interpolator|fdesign\\.hilbert|fdesign\\.highpass|fdesign\\.halfband|fdesign\\.fracdelay|fdesign\\.differentiator|fdesign\\.decimator|fdesign\\.ciccomp|fdesign\\.bandstop|fdesign\\.bandpass|fdesign\\.arbmagnphase|fdesign\\.arbmag|fdesign|fdatool|fcfwrite|farrow|euclidfactors|equiripple|ellip|double|disp|dfilt\\.wdfallpass|dfilt\\.scalar|dfilt\\.parallel|dfilt\\.latticemamin|dfilt\\.latticemamax|dfilt\\.latticearma|dfilt\\.latticear|dfilt\\.latticeallpass|dfilt\\.dfsymfir|dfilt\\.dffirt|dfilt\\.dffir|dfilt\\.dfasymfir|dfilt\\.df2tsos|dfilt\\.df2t|dfilt\\.df2sos|dfilt\\.df2|dfilt\\.df1tsos|dfilt\\.df1t|dfilt\\.df1sos|dfilt\\.df1|dfilt\\.cascadewdfallpass|dfilt\\.cascadeallpass|dfilt\\.cascade|dfilt\\.calatticepc|dfilt\\.calattice|dfilt\\.allpass|dfilt|designopts|designmethods|design|denormalize|cumsec|cost|convert|coewrite|coeread|coeffs|cl2tf|cheby2|cheby1|ca2tf|butter|block|autoscale|allpassshiftc|allpassshift|allpassrateup|allpasslp2xn|allpasslp2xc|allpasslp2mbc|allpasslp2mb|allpasslp2lp|allpasslp2hp|allpasslp2bsc|allpasslp2bs|allpasslp2bpc|allpasslp2bp|allpassbpc2bpc|adaptfilt\\.ufdaf|adaptfilt\\.tdafdft|adaptfilt\\.tdafdct|adaptfilt\\.swrls|adaptfilt\\.swftf|adaptfilt\\.ss|adaptfilt\\.se|adaptfilt\\.sd|adaptfilt\\.rls|adaptfilt\\.qrdrls|adaptfilt\\.qrdlsl|adaptfilt\\.pbufdaf|adaptfilt\\.pbfdaf|adaptfilt\\.nlms|adaptfilt\\.lsl|adaptfilt\\.lms|adaptfilt\\.hswrls|adaptfilt\\.hrls|adaptfilt\\.gal|adaptfilt\\.ftf|adaptfilt\\.filtxlms|adaptfilt\\.fdaf|adaptfilt\\.dlms|adaptfilt\\.blmsfft|adaptfilt\\.blms|adaptfilt\\.bap|adaptfilt\\.apru|adaptfilt\\.ap|adaptfilt\\.adjlms|adaptfilt)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044129,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
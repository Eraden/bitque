
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
      regex: Regex::new("\\b(zplane|zp2tf|zp2ss|zp2sos|zerophase|yulewalk|xcov|xcorr2|xcorr|wvtool|wintool|window|vco|upsample|upfirdn|unwrap|uencode|udecode|tukeywin|tripuls|triang|tfestimate|tf2zpk|tf2zp|tf2ss|tf2sos|tf2latc|taylorwin|strips|stmcb|stepz|ss2zp|ss2tf|ss2sos|square|sptool|spectrum\\.yulear|spectrum\\.welch|spectrum\\.periodogram|spectrum\\.music|spectrum\\.mtm|spectrum\\.mcov|spectrum\\.eigenvector|spectrum\\.cov|spectrum\\.burg|spectrum|spectrogram|sosfilt|sos2zp|sos2tf|sos2ss|sos2cell|sinc|sigwin|sgolayfilt|sgolay|seqperiod|schurrc|sawtooth|rootmusic|rooteig|rlevinson|residuez|resample|rectwin|rectpuls|rceps|rc2poly|rc2lar|rc2is|rc2ac|pyulear|pwelch|pulstran|prony|pow2db|polystab|polyscale|poly2rc|poly2lsf|poly2ac|pmusic|pmtm|pmcov|phasez|phasedelay|periodogram|peig|pcov|pburg|parzenwin|nuttallwin|mscohere|modulate|medfilt1|maxflat|lsf2poly|lpc|lp2lp|lp2hp|lp2bs|lp2bp|levinson|latcfilt|latc2tf|lar2rc|kaiserord|kaiser|is2rc|invfreqz|invfreqs|intfilt|interp|impz|impinvar|ifft2|ifft|idct|icceps|hilbert|hann|hamming|grpdelay|goertzel|gmonopuls|gausswin|gaussfir|gauspuls|fvtool|freqz|freqspace|freqs|flattopwin|firrcos|firpmord|firpm|firls|fircls1|fircls|fir2|fir1|findpeaks|filtstates\\.dfiir|filtstates|filtic|filtfilt|filternorm|filter2|filter|fftshift|fftfilt|fft2|fft|fdatool|eqtflength|ellipord|ellipap|ellip|dspfwiz|dspdata\\.pseudospectrum|dspdata\\.psd|dspdata\\.msspectrum|dspdata|dpsssave|dpssload|dpssdir|dpssclear|dpss|downsample|diric|digitrevorder|dftmtx|dfilt\\.statespace|dfilt\\.scalar|dfilt\\.parallel|dfilt\\.latticemamin|dfilt\\.latticemamax|dfilt\\.latticearma|dfilt\\.latticear|dfilt\\.latticeallpass|dfilt\\.fftfir|dfilt\\.dfsymfir|dfilt\\.dffirt|dfilt\\.dffir|dfilt\\.dfasymfir|dfilt\\.df2tsos|dfilt\\.df2t|dfilt\\.df2sos|dfilt\\.df2|dfilt\\.df1tsos|dfilt\\.df1t|dfilt\\.df1sos|dfilt\\.df1|dfilt\\.delay|dfilt\\.cascade|dfilt|demod|deconv|decimate|dct|db2pow|czt|cpsd|cplxpair|cov|corrmtx|corrcoef|convmtx|conv2|conv|chirp|cheby2|cheby1|chebwin|cheb2ord|cheb2ap|cheb1ord|cheb1ap|cfirpm|cell2sos|cconv|cceps|buttord|butter|buttap|buffer|bohmanwin|blackmanharris|blackman|bitrevorder|bilinear|besself|besselap|bartlett|barthannwin|aryule|armcov|arcov|arburg|angle|ac2rc|ac2poly|abs)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044149,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
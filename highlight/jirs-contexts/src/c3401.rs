
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
      regex: Regex::new("\\b(abs|addevent|addsample|addsampletocollection|addts|angle|conv|conv2|convn|corrcoef|cov|cplxpair|ctranspose|cumtrapz|deconv|del2|delevent|delsample|delsamplefromcollection|detrend|diff|fft|fft2|fftn|fftshift|fftw|filter|filter2|getabstime|getdatasamplesize|getinterpmethod|getqualitydesc|getsampleusingtime|gettimeseriesnames|gettsafteratevent|gettsafterevent|gettsatevent|gettsbeforeatevent|gettsbeforeevent|gettsbetweenevents|gradient|idealfilter|ifft|ifft2|ifftn|ifftshift|iqr|max|mean|median|min|mldivide|mode|mrdivide|removets|resample|setabstime|setinterpmethod|settimeseriesnames|std|synchronize|timeseries|trapz|tscollection|tsdata.event|tsprops|tstool|var)\\b"),
      scope: vec![
        Scope {
            a: 52640025421217792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
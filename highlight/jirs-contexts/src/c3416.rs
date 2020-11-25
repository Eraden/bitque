
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
      regex: Regex::new("\\b(wgn|vitdec|vec2mat|varlms|syndtable|symerr|stdchan|ssbmod|ssbdemod|signlms|shift2mask|seqgen\\.pn|seqgen|semianalytic|scatterplot|rsgenpoly|rsencof|rsenc|rsdecof|rsdec|rls|ricianchan|reset|rectpulse|rcosine|rcosiir|rcosflt|rcosfir|rayleighchan|randsrc|randintrlv|randint|randerr|randdeintrlv|quantiz|qfuncinv|qfunc|qammod|qamdemod|pskmod|pskdemod|primpoly|poly2trellis|pmmod|pmdemod|plot|pammod|pamdemod|oqpskmod|oqpskdemod|oct2dec|normlms|noisebw|muxintrlv|muxdeintrlv|mskmod|mskdemod|modnorm|modem\\.qammod|modem\\.qamdemod|modem\\.pskmod|modem\\.pskdemod|modem\\.pammod|modem\\.pamdemod|modem\\.oqpskmod|modem\\.oqpskdemod|modem\\.mskmod|modem\\.mskdemod|modem\\.genqammod|modem\\.genqamdemod|modem\\.dpskmod|modem\\.dpskdemod|modem|mlseeq|mldivide|minpol|matintrlv|matdeintrlv|mask2shift|marcumq|log|lms|lloyds|lineareq|istrellis|isprimitive|iscatastrophic|intrlv|intdump|ifft|huffmanenco|huffmandict|huffmandeco|hilbiir|helscanintrlv|helscandeintrlv|helintrlv|heldeintrlv|hank2sys|hammgen|gray2bin|gfweight|gftuple|gftrunc|gftable|gfsub|gfroots|gfrepcov|gfrank|gfprimfd|gfprimdf|gfprimck|gfpretty|gfmul|gfminpol|gflineq|gffilter|gfdiv|gfdeconv|gfcosets|gfconv|gfadd|gf|genqammod|genqamdemod|gen2par|fskmod|fskdemod|fmmod|fmdemod|finddelay|filter|fft|fec\\.ldpcenc|fec\\.ldpcdec|eyediagram|equalize|encode|dvbs2ldpc|dpskmod|dpskdemod|dpcmopt|dpcmenco|dpcmdeco|doppler\\.rounded|doppler\\.rjakes|doppler\\.jakes|doppler\\.gaussian|doppler\\.flat|doppler\\.bigaussian|doppler\\.ajakes|doppler|distspec|dftmtx|dfe|deintrlv|decode|de2bi|cyclpoly|cyclgen|cosets|convmtx|convintrlv|convenc|convdeintrlv|compand|commscope\\.eyediagram|commscope|cma|bsc|biterr|bin2gray|bi2de|bertool|bersync|berfit|berfading|berconfint|bercoding|berawgn|bchnumerr|bchgenpoly|bchenc|bchdec|awgn|arithenco|arithdeco|ammod|amdemod|alignsignals|algintrlv|algdeintrlv)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044123,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
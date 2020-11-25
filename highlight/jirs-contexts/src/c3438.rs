
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
      regex: Regex::new("\\b(vec2ind|tribas|trainscg|trains|trainrp|trainr|trainoss|trainlm|traingdx|traingdm|traingda|traingd|traincgp|traincgf|traincgb|trainc|trainbr|trainbfgc|trainbfg|trainb|train|tansig|sse|srchhyb|srchgol|srchcha|srchbre|srchbac|sp2narx|softmax|sim|setx|seq2con|scalprod|satlins|satlin|revert|removerows|removeconstantrows|randtop|rands|randnr|randnc|radbas|quant|purelin|processpca|postreg|poslin|pnormc|plotvec|plotv|plotsom|plotpv|plotperf|plotpc|plotes|plotep|plotbr|normr|normprod|normc|nntool|nnt2som|nnt2rb|nnt2p|nnt2lvq|nnt2lin|nnt2hop|nnt2ff|nnt2elm|nnt2c|nftool|newsom|newrbe|newrb|newpnn|newp|newnarxsp|newnarx|newlvq|newlrn|newlind|newlin|newhop|newgrnn|newfftd|newff|newelm|newdtdnn|newcf|newc|network|netsum|netprod|netinv|negdist|mseregec|msereg|mse|minmax|midpoint|maxlinlr|mapstd|mapminmax|mandist|mae|logsig|linkdist|learnwh|learnsom|learnpn|learnp|learnos|learnlv2|learnlv1|learnk|learnis|learnhd|learnh|learngdm|learngd|learncon|initzero|initwb|initnw|initlay|initcon|init|ind2vec|hintonwb|hintonw|hextop|hardlims|hardlim|gridtop|getx|gensim|fixunknowns|errsurf|dotprod|dividerand|divideint|divideind|divideblock|dist|display|disp|convwf|concur|con2seq|compet|combvec|calcperf|calcpd|calcjx|calcjejj|calcgx|boxdist|adapt)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044144,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
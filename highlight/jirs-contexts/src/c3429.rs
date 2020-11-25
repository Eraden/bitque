
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
      regex: Regex::new("\\b(zmf|writefis|trimf|trapmf|surfview|subclust|smf|sigmf|showrule|showfis|sffis|setfis|ruleview|ruleedit|rmvar|rmmf|readfis|psigmf|probor|plotmf|plotfis|pimf|parsrule|newfis|mfedit|mf2mf|mam2sug|getfis|gensurf|genfis3|genfis2|genfis1|gbellmf|gaussmf|gauss2mf|fuzzy|fuzblock|fuzarith|findcluster|fcm|evalmf|evalfis|dsigmf|defuzz|convertfis|anfisedit|anfis|addvar|addrule|addmf)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044136,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
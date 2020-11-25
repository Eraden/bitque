
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
      regex: Regex::new("(&|\\*|\\+|-|/|÷|\\^)"),
      scope: vec![
        Scope {
            a: 52636628119191560,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=|≠|>|<|≥|>=|≤|<=)"),
      scope: vec![
        Scope {
            a: 52636628119257096,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?ix)\\b\n\t(and|or|div|mod|as|not\n\t|(a\\s+)?(ref(\\s+to)?|reference\\s+to)\n\t|equal(s|\\s+to)|contains?|comes\\s+(after|before)|(start|begin|end)s?\\s+with\n\t)\n\\b"),
      scope: vec![
        Scope {
            a: 52636628119322632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?ix)\\b\n\t(is(n\'t|\\s+not)?(\\s+(equal(\\s+to)?|(less|greater)\\s+than(\\s+or\\s+equal(\\s+to)?)?|in|contained\\s+by))?\n\t|does(n\'t|\\s+not)\\s+(equal|come\\s+(before|after)|contain)\n\t)\n\\b"),
      scope: vec![
        Scope {
            a: 52636628119322632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:some|every|whose|where|that|id|index|\\d+(st|nd|rd|th)|first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|last|front|back|middle|named|beginning|end|from|to|thr(u|ough)|before|(front|back|beginning|end)\\s+of|after|behind|in\\s+(front|back|beginning|end)\\s+of)\\b"),
      scope: vec![
        Scope {
            a: 52636628110147592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:continue|return|exit(\\s+repeat)?)\\b"),
      scope: vec![
        Scope {
            a: 52636636708536328,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:about|above|after|against|and|apart\\s+from|around|as|aside\\s+from|at|back|before|beginning|behind|below|beneath|beside|between|but|by|considering|contain|contains|contains|copy|div|does|eighth|else|end|equal|equals|error|every|false|fifth|first|for|fourth|from|front|get|given|global|if|ignoring|in|instead\\s+of|into|is|it|its|last|local|me|middle|mod|my|ninth|not|of|on|onto|or|out\\s+of|over|prop|property|put|ref|reference|repeat|returning|script|second|set|seventh|since|sixth|some|tell|tenth|that|the|then|third|through|thru|timeout|times|to|transaction|true|try|until|where|while|whose|with|without)\\b"),
      scope: vec![
        Scope {
            a: 52636787013058560,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
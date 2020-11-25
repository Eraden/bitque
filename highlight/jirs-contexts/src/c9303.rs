
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 845056290324480,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845056290324480,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(WAR|WARNING)\\b"),
      scope: vec![
        Scope {
            a: 50104723402915840,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^(INVITE|ACK|PUBLISH|OPTIONS|CANCEL|BYE|SUBSCRIBE|NOTIFY|INFO|REFER|UPDATE|MESSAGE) sip.+(?m:$)|SIP/2.0 180 Ringing|SIP/2.0 200 OK)"),
      scope: vec![
        Scope {
            a: 49258876838608896,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-----\\>step.+(?m:$)"),
      scope: vec![
        Scope {
            a: 61925255085555712,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".+\\*{3,}.+"),
      scope: vec![
        Scope {
            a: 51509920738050048,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(INFO|VERBOSE)\\b"),
      scope: vec![
        Scope {
            a: 51509920738050048,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bNOTICE\\b"),
      scope: vec![
        Scope {
            a: 61925366754705408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ERR|ERROR)\\b"),
      scope: vec![
        Scope {
            a: 50103314653642752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\[.+?\\]"),
      scope: vec![
        Scope {
            a: 55454586222870528,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(--->|<---)( SIP read from | Transmitting \\(NAT\\) to | SIP transmit to).+(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711012032512,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\<— SIP read from .+ —\\>"),
      scope: vec![
        Scope {
            a: 59955200831520768,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\<— Transmitting \\(NAT\\) .+ —\\>"),
      scope: vec![
        Scope {
            a: 59955089162371072,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(SEND\\:|recv\\:).+\\>"),
      scope: vec![
        Scope {
            a: 59955089162371072,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^Call-ID\\:.+(?m:$)"),
      scope: vec![
        Scope {
            a: 59955089162371072,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(User-Agent|Server)\\:.+(?m:$)"),
      scope: vec![
        Scope {
            a: 59392130630615040,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(AMI Send\\:|AMI Recv\\:|m\\=).+"),
      scope: vec![
        Scope {
            a: 59955110637207552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*<conference-info.+"),
      scope: vec![
        Scope {
            a: 59964954702249984,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
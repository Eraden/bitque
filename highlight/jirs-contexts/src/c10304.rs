
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
      regex: Regex::new("\\b(?:aes128\\-ctr|aes128\\-gcm|aes128\\-gcm@openssh\\.com|aes192\\-ctr|aes256\\-ctr|aes256\\-gcm|aes256\\-gcm@openssh\\.com|camellia128\\-cbc|camellia128\\-ctr|camellia192\\-cbc|camellia192\\-ctr|camellia256\\-cbc|camellia256\\-ctr|chacha20\\-poly1305|chacha20\\-poly1305@openssh\\.com|twofish\\-ctr|twofish128\\-ctr|twofish192\\-ctr|twofish256\\-ctr)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 61925255255490560,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:3des|3des\\-cbc|3des\\-ctr|aes128\\-cbc|aes192\\-cbc|aes256\\-cbc|arcfour|arcfour128|arcfour256|blowfish\\-cbc|blowfish\\-ctr|cast128\\-cbc|cast128\\-ctr|des|des\\-cbc|des\\-cbc\\-ssh1|idea\\-cbc|idea\\-ctr|none|rijndael\\-cbc@lysator\\.liu\\.se|rijndael128\\-cbc|rijndael192\\-cbc|rijndael256\\-cbc|serpent128\\-cbc|serpent128\\-ctr|serpent192\\-cbc|serpent192\\-ctr|serpent256\\-cbc|serpent256\\-ctr|twofish128\\-cbc|twofish192\\-cbc|twofish256\\-cbc)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 50104723572850688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
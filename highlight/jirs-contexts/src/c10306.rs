
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
      regex: Regex::new("\\b(?:ecdsa\\-sha2\\-1\\.3\\.132\\.0\\.10|ecdsa\\-sha2\\-nistp256|ecdsa\\-sha2\\-nistp256\\-cert\\-v01@openssh\\.com|ecdsa\\-sha2\\-nistp384|ecdsa\\-sha2\\-nistp384\\-cert\\-v01@openssh\\.com|ecdsa\\-sha2\\-nistp521|ecdsa\\-sha2\\-nistp521\\-cert\\-v01@openssh\\.com|rsa\\-sha2\\-256|rsa\\-sha2\\-256\\-cert\\-v01@openssh\\.com|rsa\\-sha2\\-512|rsa\\-sha2\\-512\\-cert\\-v01@openssh\\.com|sk\\-ecdsa\\-sha2\\-nistp256\\-cert\\-v01@openssh\\.com|sk\\-ecdsa\\-sha2\\-nistp256@openssh\\.com|ssh\\-ed25519|ssh\\-ed25519\\-cert\\-v01@openssh\\.com|ssh\\-rsa|ssh\\-rsa\\-cert\\-v01@openssh\\.com|ssh\\-rsa\\-sha256@ssh\\.com|x509v3\\-sign\\-rsa|x509v3\\-sign\\-rsa\\-sha256@ssh\\.com|x509v3\\-ssh\\-rsa)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 61925375514705920,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:ssh\\-dss|ssh\\-dss\\-cert\\-v00@openssh\\.com|ssh\\-dss\\-cert\\-v01@openssh\\.com|ssh\\-rsa\\-cert\\-v00@openssh\\.com|x509v3\\-sign\\-dss)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 50104723572981760,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
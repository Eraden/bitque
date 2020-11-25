
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
      regex: Regex::new("\\b(?:hmac\\-sha1|hmac\\-sha2\\-56|hmac\\-sha2\\-224|hmac\\-sha2\\-256|hmac\\-sha2\\-384|hmac\\-sha2\\-512|hmac\\-sha3\\-256|hmac\\-sha3\\-384|hmac\\-sha3\\-512|hmac\\-sha256|hmac\\-sha256\\-96@ssh\\.com|hmac\\-sha256@ssh\\.com|hmac\\-sha512|hmac\\-sha512@ssh\\.com|umac\\-64@openssh\\.com|umac\\-128@openssh\\.com|hmac\\-sha1\\-etm@openssh\\.com|hmac\\-sha2\\-256\\-96\\-etm@openssh\\.com|hmac\\-sha2\\-512\\-96\\-etm@openssh\\.com|hmac\\-sha2\\-256\\-etm@openssh\\.com|hmac\\-sha2\\-512\\-etm@openssh\\.com|umac\\-32@openssh\\.com|umac\\-64\\-etm@openssh\\.com|umac\\-96@openssh\\.com|umac\\-128\\-etm@openssh\\.com|aes128\\-gcm|aes256\\-gcm)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 61925255255687168,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:hmac\\-md5|hmac\\-md5\\-96|hmac\\-md5\\-96\\-etm@openssh\\.com|hmac\\-md5\\-etm@openssh\\.com|hmac\\-ripemd|hmac\\-ripemd160|hmac\\-ripemd160\\-etm@openssh\\.com|hmac\\-ripemd160@openssh\\.com|hmac\\-sha1\\-96|hmac\\-sha1\\-96\\-etm@openssh\\.com|hmac\\-sha2\\-256\\-96|hmac\\-sha2\\-512\\-96|none)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 50104723573047296,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
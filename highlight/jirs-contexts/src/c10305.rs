
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
      regex: Regex::new("\\b(?:curve25519\\-sha256|curve25519\\-sha256@libssh\\.org|curve448\\-sha512|diffie\\-hellman\\-group\\-exchange\\-sha256|diffie\\-hellman\\-group\\-exchange\\-sha256@ssh\\.com|diffie\\-hellman\\-group\\-exchange\\-sha512@ssh\\.com|diffie\\-hellman\\-group14\\-sha256|diffie\\-hellman\\-group14\\-sha256@ssh\\.com|diffie\\-hellman\\-group15\\-sha256|diffie\\-hellman\\-group15\\-sha256@ssh\\.com|diffie\\-hellman\\-group15\\-sha384@ssh\\.com|diffie\\-hellman\\-group15\\-sha512|diffie\\-hellman\\-group16\\-sha256|diffie\\-hellman\\-group16\\-sha384@ssh\\.com|diffie\\-hellman\\-group16\\-sha512|diffie\\-hellman\\-group16\\-sha512@ssh\\.com|diffie\\-hellman\\-group17\\-sha512|diffie\\-hellman\\-group18\\-sha512|diffie\\-hellman\\-group18\\-sha512@ssh\\.com|ecdh\\-sha2\\-1\\.3\\.132\\.0\\.10|ecdh\\-sha2\\-curve25519|ecdh\\-sha2\\-nistb233|ecdh\\-sha2\\-nistb409|ecdh\\-sha2\\-nistk163|ecdh\\-sha2\\-nistk233|ecdh\\-sha2\\-nistk283|ecdh\\-sha2\\-nistk409|ecdh\\-sha2\\-nistp192|ecdh\\-sha2\\-nistp224|ecdh\\-sha2\\-nistp256|ecdh\\-sha2\\-nistp384|ecdh\\-sha2\\-nistp521|ecdh\\-sha2\\-nistt571|ext\\-info\\-c|ext\\-info\\-s|gss\\-group14\\-sha256\\-toWM5Slw5Ew8Mqkay\\+al2g==|gss\\-group15\\-sha512\\-toWM5Slw5Ew8Mqkay\\+al2g==|kexguess2@matt\\.ucc\\.asn\\.au|rsa1024\\-sha1|rsa2048\\-sha256|sntrup4591761x25519\\-sha512@tinyssh\\.org)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 61925255255556096,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:diffie\\-hellman\\-group\\-exchange\\-sha1|diffie\\-hellman\\-group1\\-sha1|diffie\\-hellman\\-group14\\-sha1|gss\\-gex\\-sha1\\-|gss\\-gex\\-sha1\\-toWM5Slw5Ew8Mqkay\\+al2g==|gss\\-group1\\-sha1\\-|gss\\-group1\\-sha1\\-toWM5Slw5Ew8Mqkay\\+al2g==|gss\\-group14\\-sha1\\-|gss\\-group14\\-sha1\\-toWM5Slw5Ew8Mqkay\\+al2g==)(?=[,\\s\\\"])"),
      scope: vec![
        Scope {
            a: 50104723572916224,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
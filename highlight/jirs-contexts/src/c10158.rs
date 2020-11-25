
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
      regex: Regex::new("(?i)(?=(?ix)(?:\n  # https://mimesniff.spec.whatwg.org/#javascript-mime-type\n  (?:application|text)/(?:x-)?(?:java|ecma)script\n  | text/javascript1\\.[0-5]\n  | text/jscript\n  | text/livescript\n)(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\'(?ix)(?:\n  # https://mimesniff.spec.whatwg.org/#javascript-mime-type\n  (?:application|text)/(?:x-)?(?:java|ecma)script\n  | text/javascript1\\.[0-5]\n  | text/jscript\n  | text/livescript\n)\'|\"(?ix)(?:\n  # https://mimesniff.spec.whatwg.org/#javascript-mime-type\n  (?:application|text)/(?:x-)?(?:java|ecma)script\n  | text/javascript1\\.[0-5]\n  | text/jscript\n  | text/livescript\n)\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10153 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=module(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\'module\'|\"module\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10153 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=>|\'\'|\"\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10153 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(?=text/html(?!(?:[^\\s<>/\'\'\"]|/(?!>))+)|\'text/html\'|\"text/html\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10152 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10156 }),
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10181 }),
    ]),
      with_prototype: None
    }),
]
} }
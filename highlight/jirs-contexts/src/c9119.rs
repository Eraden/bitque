
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
      regex: Regex::new("\\s*(#)(?={.*}(?x:\n    (:{1,2})(?:before|after|first-line|first-letter) # CSS1 & CSS2 require : or ::\n  | (::)(-(?:moz|ms|webkit)-)?(?:(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)) # CSS3 requires ::\n)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9024 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(#)(?={.*}:\\b(active|any-link|blank|checked|current|default|defined|disabled|drop|empty|enabled|first|first-child|first-of-type|fullscreen|future|focus|focus-visible|focus-within|host|hover|indeterminate|in-range|invalid|last-child|last-of-type|left|link|local-link|only-child|only-of-type|optional|out-of-range|past|placeholder-shown|read-only|read-write|required|right|root|scope|target|target-within|user-invalid|valid|visited)\\b(?![-]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9025 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(#)(?={.*}:(\\b(dir|lang|matches|not|has|drop|nth-last-child|nth-child|nth-last-of-type|nth-of-type)\\b)\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9026 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(#)(?={.*}:(-(moz|ms|webkit)-)(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9027 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(#)(?={.*}(?![-:]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9028 }),
    ]),
      with_prototype: None
    }),
]
} }
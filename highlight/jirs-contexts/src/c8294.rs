
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
      regex: Regex::new("\\s*(@)({)([a-zA-Z0-9_-][\\w-]*)(})(?=(?x:\n    (:{1,2})(?:before|after|first-line|first-letter) # CSS1 & CSS2 require : or ::\n  | (::)(-(?:moz|ms|webkit)-)?(?:(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)) # CSS3 requires ::\n)\\b)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087300067328,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514555,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 34621422135410688,
        },
    ]),(3, vec![
        Scope {
            a: 55452159557959680,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8202 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(@)({)([a-zA-Z0-9_-][\\w-]*)(})(?=:\\b(active|any-link|blank|checked|current|default|defined|disabled|drop|empty|enabled|first|first-child|first-of-type|fullscreen|future|focus|focus-visible|focus-within|host|hover|indeterminate|in-range|invalid|last-child|last-of-type|left|link|local-link|only-child|only-of-type|optional|out-of-range|past|placeholder-shown|read-only|read-write|required|right|root|scope|target|target-within|user-invalid|valid|visited)\\b(?![-]))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087300067328,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514555,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 34621422135410688,
        },
    ]),(3, vec![
        Scope {
            a: 55452159557959680,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8203 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(@)({)([a-zA-Z0-9_-][\\w-]*)(})(?=:(\\b(dir|lang|matches|not|has|drop|nth-last-child|nth-child|nth-last-of-type|nth-of-type)\\b)\\()"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087300067328,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514555,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 34621422135410688,
        },
    ]),(3, vec![
        Scope {
            a: 55452159557959680,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8204 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(@)({)([a-zA-Z0-9_-][\\w-]*)(})(?=:(-(moz|ms|webkit)-)(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087300067328,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514555,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 34621422135410688,
        },
    ]),(3, vec![
        Scope {
            a: 55452159557959680,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8205 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(@)({)([a-zA-Z0-9_-][\\w-]*)(})(?=(?![-:]))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087300067328,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514555,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 34621422135410688,
        },
    ]),(3, vec![
        Scope {
            a: 55452159557959680,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956395,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8206 }),
    ]),
      with_prototype: None
    }),
]
} }
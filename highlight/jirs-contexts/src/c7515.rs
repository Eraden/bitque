
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7547 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7508 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7507 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7504 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7574 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7527 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7532 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7533 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7534 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7506 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7505 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7576 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7509 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\,"),
      scope: vec![
        Scope {
            a: 47288620728516608,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[)}]"),
      scope: vec![
        Scope {
            a: 50103314683854961,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) # ignore whitespace in this regex\n  \\b(?:\n    common |\n    partition |\n    active |\n    asm |\n    class |\n    union |\n    enum |\n    typedef |\n    template |\n    this |\n    resource |\n    goto |\n    inline |\n    noinline |\n    public |\n    static |\n    extern |\n    external |\n    interface |\n    long |\n    short |\n    half |\n    fixed |\n    unsigned |\n    superp |\n    input |\n    output |\n    hvec2 |\n    hvec3 |\n    hvec4 |\n    fvec2 |\n    fvec3 |\n    fvec4 |\n    sampler3DRect |\n    filter |\n    sizeof |\n    cast |\n    namespace |\n    using\n  )\\b\n"),
      scope: vec![
        Scope {
            a: 50103314806734961,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
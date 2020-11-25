
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
      regex: Regex::new("((@)selector)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466264064,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322318029,
            b: 16044073672507392,
        },
    ]),(3, vec![
        Scope {
            a: 47288629322318029,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3886 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(id)\\s*(?=<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466264064,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3887 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bid\\b(\\s|\\n)?"),
      scope: vec![
        Scope {
            a: 48414576496148537,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(IBOutlet|IBAction|BOOL|SEL|id|unichar|IMP|Class)\\b"),
      scope: vec![
        Scope {
            a: 48414576466264064,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(class|protocol)\\b"),
      scope: vec![
        Scope {
            a: 48414576466264064,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322318029,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bC(I(Sampler|Co(ntext|lor)|Image(Accumulator)?|PlugIn(Registration)?|Vector|Kernel|Filter(Generator|Shape)?)|A(Renderer|MediaTiming(Function)?|BasicAnimation|ScrollLayer|Constraint(LayoutManager)?|T(iledLayer|extLayer|rans(ition|action))|OpenGLLayer|PropertyAnimation|KeyframeAnimation|Layer|A(nimation(Group)?|ction)))\\b"),
      scope: vec![
        Scope {
            a: 61925366828171321,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bC(G(Float|Point|Size|Rect)|IFormat|AConstraintAttribute)\\b"),
      scope: vec![
        Scope {
            a: 61925375418105913,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
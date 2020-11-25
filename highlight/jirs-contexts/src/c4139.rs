
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
      regex: Regex::new("^(?!\\s*\\*)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4067 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\*)\\s*(@access)\\s+((var|public|private|protected)|(.+))\\s*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038778,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787086327866,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414439027376128,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50103314728748134,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\*)(?!/)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038778,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((https?|s?ftp|ftps|file|smb|afp|nfs|(x-)?man|gopher|txmt)://|mailto:)[-:@a-zA-Z0-9_.~%+/?=&#]*[-@a-zA-Z0-9_~%+/=&#]"),
      scope: vec![
        Scope {
            a: 114280588597985338,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@xlink)\\s+(.+)\\s*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787086327866,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 114280588597985338,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\@(a(bstract|pi|uthor)|c(ategory|opyright)|example|global|i(nternal|gnore)|li(cense|nk)|pa(ckage|ram)|return|s(ee|ince|tatic|ubpackage)|t(hrows|odo)|v(ar|ersion)|uses|deprecated|fi(nal|lesource)|property(-(read|write))?|method|source)\\b"),
      scope: vec![
        Scope {
            a: 52636787086327866,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(@inheritdoc)\\}"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787086327866,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\@(author|after(?:Class)?|backup(?:Globals|StaticAttributes)|before(?:Class)?|codeCoverageIgnore*|covers(?:DefaultClass|Nothing)?|dataProvider|depends|doesNotPerformAssertions|expectedException(?:Code|Message(?:RegExp)?)?|group|large|medium|preserveGlobalState|requires|run(TestsInSeparateProcesses|InSeparateProcess)|small|test(dox|With)?|ticket|uses)\\b"),
      scope: vec![
        Scope {
            a: 52636787087704122,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(@(link)).+?\\}"),
      scope: vec![
        Scope {
            a: 46444230200067174,
            b: 16325548649218048,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787086327866,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }

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
      regex: Regex::new("\\b((sub|di)?graph)\\b\\s+((cluster_)?\\w+)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576464625664,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130665676832,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46445080556339200,
            b: 0,
        },
        Scope {
            a: 49259830368075808,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(node|edge|graph|digraph|subgraph|strict)\\b"),
      scope: vec![
        Scope {
            a: 48414576464625664,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(bottomlabel|color|comment|distortion|fillcolor|fixedsize|fontcolor|fontname|fontsize|group|height|label|layer|orientation|peripheries|regular|shape|shapefile|sides|skew|style|toplabel|URL|width|z)\\b"),
      scope: vec![
        Scope {
            a: 61925409733739210,
            b: 9007199254740992,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(arrowhead|arrowsize|arrowtail|color|comment|constraint|decorate|dir|fontcolor|fontname|fontsize|headlabel|headport|headURL|label|labelangle|labeldistance|labelfloat|labelcolor|labelfontname|labelfontsize|layer|lhead|ltail|minlen|samehead|sametail|style|taillabel|tailport|tailURL|weight)\\b"),
      scope: vec![
        Scope {
            a: 61925409733739211,
            b: 9007199254740992,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(bgcolor|center|clusterrank|color|comment|compound|concentrate|fillcolor|fontname|fontpath|fontsize|label|labeljust|labelloc|layers|margin|mclimit|nodesep|nslimit|nslimit1|ordering|orientation|page|pagedir|quantum|rank|rankdir|ranksep|ratio|remincross|rotate|samplepoints|searchsize|size|style|URL)\\b"),
      scope: vec![
        Scope {
            a: 61925409733739208,
            b: 9007199254740992,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-[->]"),
      scope: vec![
        Scope {
            a: 47288603588165664,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288620734808096,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 9007199254740992,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1976 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1979 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1980 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1981 })),
]
} }
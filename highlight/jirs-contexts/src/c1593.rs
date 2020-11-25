
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
  prototype: Some(
    ContextId {
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\d+(\\.)\\d+([eE][-+]?\\d+)?\\b"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 5629499534213120,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397908,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:((2\\#)[0-1]+)       # binary\n  | ((8\\#)[0-7]+)       # octal\n  | ((10\\#)[0-9]+)      # decimal\n  | ((16\\#)[\\da-fA-F]+) # hexadecimal\n  | ((3\\#)[0-2]+\n  |  (4\\#)[0-3]+\n  |  (5\\#)[0-4]+\n  |  (6\\#)[0-5]+\n  |  (7\\#)[0-6]+\n  |  (9\\#)[0-8]+\n  |  (11\\#)[\\daA]+\n  |  (12\\#)[\\da-bA-B]+\n  |  (13\\#)[\\da-cA-C]+\n  |  (14\\#)[\\da-dA-D]+\n  |  (15\\#)[\\da-eA-E]+\n  |  (17\\#)[\\da-gA-G]+\n  |  (18\\#)[\\da-hA-H]+\n  |  (19\\#)[\\da-iA-I]+\n  |  (20\\#)[\\da-jA-J]+\n  |  (21\\#)[\\da-kA-K]+\n  |  (22\\#)[\\da-lA-L]+\n  |  (23\\#)[\\da-mA-M]+\n  |  (24\\#)[\\da-nA-N]+\n  |  (25\\#)[\\da-oA-O]+\n  |  (26\\#)[\\da-pA-P]+\n  |  (27\\#)[\\da-qA-Q]+\n  |  (28\\#)[\\da-rA-R]+\n  |  (29\\#)[\\da-sA-S]+\n  |  (30\\#)[\\da-tA-T]+\n  |  (31\\#)[\\da-uA-U]+\n  |  (32\\#)[\\da-vA-V]+\n  |  (33\\#)[\\da-wA-W]+\n  |  (34\\#)[\\da-xA-X]+\n  |  (35\\#)[\\da-yA-Y]+\n  |  (36\\#)[\\da-zA-Z]+) # other\n  | ((\\d+\\#)\\S+)        # illegal\n)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461741,
            b: 5629499534213120,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(3, vec![
        Scope {
            a: 59955089176461666,
            b: 5629499534213120,
        },
    ]),(4, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(5, vec![
        Scope {
            a: 59955089176461530,
            b: 5629499534213120,
        },
    ]),(6, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(7, vec![
        Scope {
            a: 59955089176461528,
            b: 5629499534213120,
        },
    ]),(8, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(9, vec![
        Scope {
            a: 59955089176461537,
            b: 5629499534213120,
        },
    ]),(10, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(11, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(12, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(13, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(14, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(15, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(16, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(17, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(18, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(19, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(20, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(21, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(22, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(23, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(24, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(25, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(26, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(27, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(28, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(29, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(30, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(31, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(32, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(33, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(34, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(35, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(36, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(37, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(38, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(39, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(40, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ]),(41, vec![
        Scope {
            a: 50103314667733012,
            b: 0,
        },
    ]),(42, vec![
        Scope {
            a: 47288629325070764,
            b: 5629499534213120,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\d+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\d\\w+"),
      scope: vec![
        Scope {
            a: 50103314667733012,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
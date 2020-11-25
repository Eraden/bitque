
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(opNeg|opCom|opPostInc|opPostDec|opCast|opAdd|opSub|opSub_r|opMul|opDiv|opDiv_r|opMod|opMod_r|opAnd|opOr|opXor|opShl|opShl_r|opShr|opShr_r|opUShr|opUShr_r|opCat|opCat_r|opEquals|opEquals|opCmp|opCmp|opCmp|opCmp|opAddAssign|opSubAssign|opMulAssign|opDivAssign|opModAssign|opAndAssign|opOrAssign|opXorAssign|opShlAssign|opShrAssign|opUShrAssign|opCatAssign|opIndex|opIndexAssign|opCall|opSlice|opSliceAssign|opPos|opAdd_r|opMul_r|opAnd_r|opOr_r|opXor_r)\\s*(?=\\(|(?m:$))"),
      scope: vec![
        Scope {
            a: 46444131367518208,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615228,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1255 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([\\p{L}_][\\p{L}0-9_]*)\\s*(?=\\(|(?m:$))"),
      scope: vec![
        Scope {
            a: 46444131367518208,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615056,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1255 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1364 }),
    ]),
      with_prototype: None
    }),
]
} }
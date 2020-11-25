
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1268 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1169 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1203 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1274 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1323 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1342 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1226 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1210 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1319 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1318 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1213 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1277 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1344 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1331 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1292 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1194 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1211 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1280 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1239 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1367 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1328 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1334 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1332 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1315 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1177 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1326 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1185 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1316 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1220 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1201 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1360 })),
]
} }
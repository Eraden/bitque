
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "OpenMP (Fortran)".to_string(),
  file_extensions: vec![],
  scope: Scope { a: 844901671501824, b: 0 },
  first_line_match: None,
  hidden: true,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("openmp_comment_start".to_string(), "(\\!|^[Cc])\\$omp\\b".to_string());
    v.insert("ident".to_string(), "[A-Za-z_][A-Za-z_0-9]*".to_string());
    v.insert("openmp_schedule_kind".to_string(), "(?xi:(static|dynamic|guided|auto|runtime))".to_string());
    v.insert("openmp_reduction_identifier".to_string(), "(?xi:(\\w+|\\+|-|\\*|.and.|.or.|.eqv.|.negv.|max|min|iand|ior|ieor))".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("openmp-clause-safelen".to_string(), ContextId { index: 7416 });
    v.insert("openmp-end-clause-nowait".to_string(), ContextId { index: 7431 });
    v.insert("openmp-clause-inbranch".to_string(), ContextId { index: 7407 });
    v.insert("openmp-list".to_string(), ContextId { index: 7434 });
    v.insert("openmp-clause-simdlen".to_string(), ContextId { index: 7419 });
    v.insert("openmp-do-simd".to_string(), ContextId { index: 7429 });
    v.insert("main".to_string(), ContextId { index: 7400 });
    v.insert("openmp-clause-if".to_string(), ContextId { index: 7406 });
    v.insert("openmp-intrinsic-functions".to_string(), ContextId { index: 7433 });
    v.insert("openmp-clause-reduction".to_string(), ContextId { index: 7415 });
    v.insert("openmp-main-scope".to_string(), ContextId { index: 7435 });
    v.insert("openmp-clause-uniform".to_string(), ContextId { index: 7420 });
    v.insert("openmp-declare-simd".to_string(), ContextId { index: 7427 });
    v.insert("openmp-clause-schedule".to_string(), ContextId { index: 7417 });
    v.insert("openmp-clause-shared-copyin".to_string(), ContextId { index: 7418 });
    v.insert("openmp-clause-notinbranch".to_string(), ContextId { index: 7410 });
    v.insert("openmp-clauses-declare-simd".to_string(), ContextId { index: 7421 });
    v.insert("openmp-clauses-do".to_string(), ContextId { index: 7422 });
    v.insert("openmp-clauses-parallel".to_string(), ContextId { index: 7423 });
    v.insert("openmp-clauses-sections".to_string(), ContextId { index: 7424 });
    v.insert("openmp-end-clauses-single".to_string(), ContextId { index: 7432 });
    v.insert("openmp-parallel".to_string(), ContextId { index: 7436 });
    v.insert("openmp-simd".to_string(), ContextId { index: 7438 });
    v.insert("openmp-clauses-single".to_string(), ContextId { index: 7426 });
    v.insert("openmp-single".to_string(), ContextId { index: 7439 });
    v.insert("openmp-target-data".to_string(), ContextId { index: 7440 });
    v.insert("openmp-clause-lastprivate".to_string(), ContextId { index: 7408 });
    v.insert("openmp-workshare".to_string(), ContextId { index: 7441 });
    v.insert("__main".to_string(), ContextId { index: 7398 });
    v.insert("__start".to_string(), ContextId { index: 7399 });
    v.insert("openmp-clause-default".to_string(), ContextId { index: 7403 });
    v.insert("openmp-clauses-simd".to_string(), ContextId { index: 7425 });
    v.insert("openmp-clause-linear".to_string(), ContextId { index: 7409 });
    v.insert("openmp-clause-aligned".to_string(), ContextId { index: 7401 });
    v.insert("openmp-clause-numthreads".to_string(), ContextId { index: 7411 });
    v.insert("openmp-clause-device".to_string(), ContextId { index: 7404 });
    v.insert("openmp-sections".to_string(), ContextId { index: 7437 });
    v.insert("openmp-end-clause-copyprivate".to_string(), ContextId { index: 7430 });
    v.insert("openmp-do".to_string(), ContextId { index: 7428 });
    v.insert("openmp-clause-ordered".to_string(), ContextId { index: 7412 });
    v.insert("openmp-clause-collapse".to_string(), ContextId { index: 7402 });
    v.insert("openmp-clause-firstprivate".to_string(), ContextId { index: 7405 });
    v.insert("openmp-clause-procbind".to_string(), ContextId { index: 7414 });
    v.insert("openmp-clause-private".to_string(), ContextId { index: 7413 });
    v
  }
} }
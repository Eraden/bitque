
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
      regex: Regex::new("(?xi)\n(?:\\b ( omp_set_num_threads\n      | omp_get_num_threads\n      | omp_get_max_threads\n      | omp_get_thread_num\n      | omp_get_num_procs\n      | omp_in_parallel\n      | omp_set_dynamic\n      | omp_get_dynamic\n      | omp_get_cancellation\n      | omp_set_nested\n      | omp_get_nested\n      | omp_set_schedule\n      | omp_get_schedule\n      | omp_get_thread_limit\n      | omp_set_max_active_levels\n      | omp_get_max_active_levels\n      | omp_get_level\n      | omp_get_ancestor_thread_num\n      | omp_get_team_size\n      | omp_get_active_level\n      | omp_get_proc_bind\n      | omp_set_default_device\n      | omp_get_default_device\n      | omp_get_num_devices\n      | omp_get_num_teams\n      | omp_get_team_num\n      | omp_is_initial_device\n      )(?=\\s*\\()\n)\n"),
      scope: vec![
        Scope {
            a: 61925255092832453,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:\\b ( omp_init_lock\n      | omp_init_nest_lock\n      | omp_destroy_lock\n      | omp_destroy_nest_lock\n      | omp_set_lock\n      | omp_set_nest_lock\n      | omp_unset_lock\n      | omp_unset_nest_lock\n      | omp_test_lock\n      | omp_test_nest_lock\n      )(?=\\s*\\()\n)\n"),
      scope: vec![
        Scope {
            a: 61925255092832453,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:\\b ( omp_get_wtime\n      | omp_get_wtick\n      )(?=\\s*\\()\n)\n"),
      scope: vec![
        Scope {
            a: 61925255092832453,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?:\\b ( OMP_CANCELLATION\n      | OMP_DEFAULT_DEVICE\n      | OMP_DISPLAY_ENV\n      | OMP_DYNAMIC\n      | OMP_MAX_ACTIVE_LEVELS\n      | OMP_NESTED\n      | OMP_NUM_THREADS\n      | OMP_PLACES\n      | OMP_PROC_BIND\n      | OMP_SCHEDULE\n      | OMP_STACKSIZE\n      | OMP_THREAD_LIMIT\n      | OMP_WAIT_POLICY\n      )\n)\n"),
      scope: vec![
        Scope {
            a: 61925409711655109,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
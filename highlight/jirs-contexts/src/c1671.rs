
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
      regex: Regex::new("(?x:\n  abs|adler32|adler32_combine|append_element|apply|atom_to_binary|atom_to_list|\n  binary_part|binary_to_atom|binary_to_existing_atom|binary_to_float|\n  binary_to_integer|binary_to_list|binary_to_term|bit_size|bitstring_to_list|\n  bump_reductions|byte_size|cancel_timer|ceil|check_old_code|check_process_code|\n  convert_time_unit|crc32|crc32_combine|date|decode_packet|delete_element|\n  delete_module|demonitor|disconnect_node|display|dist_ctrl_get_data|\n  dist_ctrl_get_data_notification|dist_ctrl_input_handler|dist_ctrl_put_data|\n  element|erase|error|exit|external_size|float|float_to_binary|float_to_list|\n  floor|fun_info|fun_to_list|function_exported|garbage_collect|get|get_cookie|\n  get_keys|get_stacktrace|group_leader|halt|hd|hibernate|insert_element|\n  integer_to_binary|integer_to_list|iolist_size|iolist_to_binary|\n  iolist_to_iovec|is_alive|is_atom|is_binary|is_bitstring|is_boolean|is_builtin|\n  is_float|is_function|is_integer|is_list|is_map|is_map_key|is_number|is_pid|\n  is_port|is_process_alive|is_record|is_reference|is_tuple|length|link|\n  list_to_atom|list_to_binary|list_to_bitstring|list_to_existing_atom|\n  list_to_float|list_to_integer|list_to_pid|list_to_port|list_to_ref|\n  list_to_tuple|load_module|load_nif|loaded|localtime|\n  localtime_to_universaltime|make_ref|make_tuple|map_get|map_size|\n  match_spec_test|max|md5|md5_final|md5_init|md5_update|memory|min|\n  module_loaded|monitor|monitor_node|monotonic_time|nif_error|node|nodes|now|\n  open_port|phash|phash2|pid_to_list|port_call|port_close|port_command|\n  port_connect|port_control|port_info|port_to_list|ports|pre_loaded|\n  process_display|process_flag|process_info|processes|purge_module|put|raise|\n  read_timer|ref_to_list|register|registered|resume_process|round|self|send|\n  send_after|send_nosuspend|set_cookie|setelement|size|spawn|spawn_link|\n  spawn_monitor|spawn_opt|split_binary|start_timer|statistics|suspend_process|\n  system_flag|system_info|system_monitor|system_profile|system_time|\n  term_to_binary|throw|time|time_offset|timestamp|tl|trace|trace_delivered|\n  trace_info|trace_pattern|trunc|tuple_size|tuple_to_list|unique_integer|\n  universaltime|universaltime_to_localtime|unlink|unregister|whereis|yield\n)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 46445879419469824,
            b: 0,
        },
        Scope {
            a: 61925255086866432,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-z][_A-Za-z\\d@]*"),
      scope: vec![
        Scope {
            a: 46445879419469824,
            b: 0,
        },
        Scope {
            a: 49258881134886912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\'"),
      scope: vec![
        Scope {
            a: 47288629349318838,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1510 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1674 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1580 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1567 })),
]
} }
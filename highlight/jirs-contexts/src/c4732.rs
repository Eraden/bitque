
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
      regex: Regex::new("(?x)\n\\b\n(\n    before_(filter|action)\n  | skip_before_(filter|action)\n  | skip_after_(filter|action)\n  | after_(filter|action)\n  | around_(filter|action)\n  | filter\n  | filter_parameter_logging\n  | layout\n  | require_dependency\n  | render\n  | render_action\n  | render_text\n  | render_file\n  | render_template\n  | render_nothing\n  | render_component\n  | render_without_layout\n  | rescue_from\n  | url_for\n  | redirect_to\n  | redirect_to_path\n  | redirect_to_url\n  | respond_to\n  | helper\n  | helper_method\n  | model\n  | observer\n  | serialize\n  | scaffold\n  | verify\n  | hide_action\n  | append_view_path\n  | prepend_view_path\n  | view_paths\n)\n(?![?!:])\n\\b\n"),
      scope: vec![
        Scope {
            a: 61925255173046339,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n\\b\n(\n    named_scope\n  | default_scope\n  | scope\n  | after_create\n  | after_destroy\n  | after_save\n  | after_update\n  | after_validation\n  | after_validation_on_create\n  | after_validation_on_update\n  | after_rollback\n  | after_(create_|destroy_|update_)?commit\n  | before_create\n  | before_destroy\n  | before_save\n  | before_update\n  | before_validation\n  | before_validation_on_create\n  | before_validation_on_update\n  | composed_of\n  | belongs_to\n  | has_one\n  | has_many\n  | has_and_belongs_to_many\n  | validate\n  | validate_on_create\n  | validates_numericality_of\n  | validate_on_update\n  | validates_acceptance_of\n  | validates_associated\n  | validates_confirmation_of\n  | validates_each\n  | validates_format_of\n  | validates_inclusion_of\n  | validates_exclusion_of\n  | validates_length_of\n  | validates_presence_of\n  | validates_size_of\n  | validates_uniqueness_of\n  | validates\n  | attr_protected\n  | attr_accessible\n  | attr_readonly\n)\n(?![?!:])\n\\b\n"),
      scope: vec![
        Scope {
            a: 61925255173111875,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n\\b\n(\n    alias_method_chain\n  | alias_attribute\n  | delegate\n  | cattr_accessor\n  | mattr_accessor\n  | class_attribute\n  | returning\n)\n\\b\n"),
      scope: vec![
        Scope {
            a: 61925255173177411,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
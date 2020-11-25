
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Fortran (Modern)".to_string(),
  file_extensions: vec!["f90".to_string(),"F90".to_string(),"f95".to_string(),"F95".to_string(),"f03".to_string(),"F03".to_string(),"f08".to_string(),"F08".to_string()],
  scope: Scope { a: 844888786599936, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("intrinsic_type_spec".to_string(), "(?xi)\n(?:\n  \\bCHARACTER\\b \\s* {{char_selector}}\n|\n  \\b(?:COMPLEX|INTEGER|LOGICAL|REAL)\\b (?:\\s* {{kind_selector}})?\n|\n  \\b DOUBLE \\s+ (?:COMPLEX|PRECISION) \\b\n)\n".to_string());
    v.insert("kind_selector".to_string(), "(?xi)\n(?:\n  \\s*\n  (?:\n    \\(  (?:\\s* kind \\s* = \\s* )?  {{int_constant_expr}} \\)\n  |\n    \\* \\s* {{int_literal_constant}}\n  )\n  \\s*\n)\n".to_string());
    v.insert("modifier".to_string(), "(?xi:elemental|pure|impure|recursive|module)".to_string());
    v.insert("construct_name".to_string(), "(?xi:{{ident}} \\s* (?=:))".to_string());
    v.insert("declaration_type_spec_part1".to_string(), "(?xi)\n(?:\n  {{intrinsic_type_spec}}\n|\n  type \\s* \\( \\s* {{intrinsic_type_spec}} \\s* \\)\n|\n  type \\s* \\( \\s* {{derived_type_spec}} \\s* \\)\n|\n  class \\s* \\( \\s* {{derived_type_spec}} \\s* \\)\n|\n  class \\s* \\( \\s* \\* \\s* \\)\n)\n".to_string());
    v.insert("char_length".to_string(), "(?xi)\n(?:\n  {{int_literal_constant}}\n|\n  \\( {{type_param_value}} \\)\n)\n".to_string());
    v.insert("formatdescr".to_string(), "(?xi:(\\d+)?  (/|pe|[aeigfxp])  (\\d+)?  \\.?  (\\d+)? )".to_string());
    v.insert("int_constant_expr".to_string(), "(?xi:\\d+|\\w+)".to_string());
    v.insert("octal_digit".to_string(), "(?xi:[0-8])".to_string());
    v.insert("construct_keyword".to_string(), "(?xi:associate|block(?!\\s+data\\b)|do|forall|if|select|where)".to_string());
    v.insert("octal_number".to_string(), "(?xi:{{octal_digit}}+)".to_string());
    v.insert("specification_expression".to_string(), "(?xi:TODO_NOT_IMPLEMENTED|\\d+)".to_string());
    v.insert("eol_comment".to_string(), "(?xi:!.*)".to_string());
    v.insert("type_param_value".to_string(), "(?xi:)\n(?:\n  {{specification_expression}}\n|\n  {{ident}} # hack\n|\n  \\*\n|\n  :\n)\n".to_string());
    v.insert("hexadecimal_string".to_string(), "(?xi:(\'|\") {{hexadecimal_number}}+ (\'|\"))".to_string());
    v.insert("valid_range_symbol".to_string(), "(?xi:[abcdefghijklmnopqrstuvwxyz_$])".to_string());
    v.insert("escapeseq".to_string(), "(?xi:\\\\ [nrtbfv0\'\'\"x\\\\] )".to_string());
    v.insert("octal_string".to_string(), "(?xi:(\'|\") {{octal_number}}+ (\'|\"))".to_string());
    v.insert("ident".to_string(), "[A-Za-z_][A-Za-z_0-9]*".to_string());
    v.insert("int_literal_constant".to_string(), "(?xi:\\d+)".to_string());
    v.insert("stmt_label".to_string(), "(?xi:\\b \\d+ \\b)".to_string());
    v.insert("hexadecimal_digit".to_string(), "(?xi:[0-9a-f])".to_string());
    v.insert("program_unit".to_string(), "(?xi:block\\s+data|function|module|program|subroutine|submodule|type|interface|type)".to_string());
    v.insert("derived_type_spec".to_string(), "(?xi)\n(?:\n  {{ident}}\n)\n".to_string());
    v.insert("hexadecimal_number".to_string(), "(?xi:{{hexadecimal_digit}}+)".to_string());
    v.insert("attribute".to_string(), "(?xi)\n(?:\n    allocatable\n  | pointer\n  | target\n  | equivalence\n  | parameter\n  | external\n  | intrinsic\n  | save\n  | optional\n  | contiguous\n  | private\n  | public\n  | protected\n)\n".to_string());
    v.insert("char_selector".to_string(), "(?xi)\n(?:\n  \\(\n    \\s*\n    (?:\n      LEN \\s* = \\s* {{type_param_value}} \\s* , \\s* KIND \\s* = \\s* {{int_constant_expr}}\n    |\n      {{type_param_value}} \\s* ,  \\s* (?:KIND \\s* =)?  \\s* {{int_constant_expr}}\n    |\n      KIND \\s* = \\s* {{int_constant_expr}}  \\s* (?: , \\s* LEN \\s* = \\s* {{type_param_value}})?\n    |\n      (?:LEN \\s* = \\s* )? {{type_param_value}}\n    )\n    \\s*\n  \\)\n|\n  \\s* \\* \\s* {{char_length}} \\s* ,? \\s*\n)\n".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("builtin-functions".to_string(), ContextId { index: 7310 });
    v.insert("call_statement".to_string(), ContextId { index: 7311 });
    v.insert("do_while_statement".to_string(), ContextId { index: 7326 });
    v.insert("implicit_range_list".to_string(), ContextId { index: 7350 });
    v.insert("string".to_string(), ContextId { index: 7383 });
    v.insert("main".to_string(), ContextId { index: 7361 });
    v.insert("#anon_bind_0".to_string(), ContextId { index: 7277 });
    v.insert("#anon_end_program_unit_1".to_string(), ContextId { index: 7288 });
    v.insert("#anon_write_statement_0".to_string(), ContextId { index: 7298 });
    v.insert("data_statement".to_string(), ContextId { index: 7322 });
    v.insert("exit_statement".to_string(), ContextId { index: 7335 });
    v.insert("include_statement".to_string(), ContextId { index: 7353 });
    v.insert("cycle_statement".to_string(), ContextId { index: 7321 });
    v.insert("format_statement".to_string(), ContextId { index: 7337 });
    v.insert("statement".to_string(), ContextId { index: 7382 });
    v.insert("program_unit_statements".to_string(), ContextId { index: 7376 });
    v.insert("preprocessor_errorwarning".to_string(), ContextId { index: 7372 });
    v.insert("#anon_declaration_type_spec_0".to_string(), ContextId { index: 7279 });
    v.insert("implicit_declaration_list".to_string(), ContextId { index: 7349 });
    v.insert("m4_macros".to_string(), ContextId { index: 7360 });
    v.insert("#anon_entry_statement_1".to_string(), ContextId { index: 7291 });
    v.insert("ieee_intrinsic_modules".to_string(), ContextId { index: 7346 });
    v.insert("fortran_2008_intrinsics".to_string(), ContextId { index: 7341 });
    v.insert("#anon_end_construct_1".to_string(), ContextId { index: 7284 });
    v.insert("inside-format-group".to_string(), ContextId { index: 7354 });
    v.insert("namelist-meta".to_string(), ContextId { index: 7363 });
    v.insert("operator_statement".to_string(), ContextId { index: 7367 });
    v.insert("use_statement".to_string(), ContextId { index: 7387 });
    v.insert("m4_macro_string".to_string(), ContextId { index: 7359 });
    v.insert("comments".to_string(), ContextId { index: 7317 });
    v.insert("else_if_then_end_of_statement".to_string(), ContextId { index: 7329 });
    v.insert("enum_statement".to_string(), ContextId { index: 7334 });
    v.insert("#anon_function-arguments_0".to_string(), ContextId { index: 7292 });
    v.insert("#anon_end_construct_3".to_string(), ContextId { index: 7286 });
    v.insert("array-initialization".to_string(), ContextId { index: 7304 });
    v.insert("#anon_array-initialization_0".to_string(), ContextId { index: 7276 });
    v.insert("inside-format-string-double".to_string(), ContextId { index: 7355 });
    v.insert("write_statement".to_string(), ContextId { index: 7388 });
    v.insert("#anon_declaration_type_spec_1".to_string(), ContextId { index: 7280 });
    v.insert("#anon_round-braces_0".to_string(), ContextId { index: 7296 });
    v.insert("common_statement".to_string(), ContextId { index: 7318 });
    v.insert("preprocessor".to_string(), ContextId { index: 7371 });
    v.insert("select_case_statement".to_string(), ContextId { index: 7380 });
    v.insert("inside-format-string-single".to_string(), ContextId { index: 7356 });
    v.insert("ieee_intrinsics".to_string(), ContextId { index: 7347 });
    v.insert("#anon_function-call_0".to_string(), ContextId { index: 7293 });
    v.insert("#anon_square-braces_0".to_string(), ContextId { index: 7297 });
    v.insert("#anon_end_program_unit_2".to_string(), ContextId { index: 7289 });
    v.insert("namelist_statement".to_string(), ContextId { index: 7364 });
    v.insert("preprocessor_include_angle_brackets".to_string(), ContextId { index: 7374 });
    v.insert("return_statement".to_string(), ContextId { index: 7378 });
    v.insert("format_string_single_quotes".to_string(), ContextId { index: 7340 });
    v.insert("case_statement".to_string(), ContextId { index: 7313 });
    v.insert("public_private_protected".to_string(), ContextId { index: 7377 });
    v.insert("print_statement".to_string(), ContextId { index: 7375 });
    v.insert("__start".to_string(), ContextId { index: 7302 });
    v.insert("#anon_end_construct_2".to_string(), ContextId { index: 7285 });
    v.insert("#anon_end_construct_0".to_string(), ContextId { index: 7283 });
    v.insert("end_program_unit".to_string(), ContextId { index: 7332 });
    v.insert("preprocessor_include".to_string(), ContextId { index: 7373 });
    v.insert("subroutine_parameter_list".to_string(), ContextId { index: 7386 });
    v.insert("literal".to_string(), ContextId { index: 7358 });
    v.insert("allocation".to_string(), ContextId { index: 7303 });
    v.insert("do_statement".to_string(), ContextId { index: 7325 });
    v.insert("expression".to_string(), ContextId { index: 7336 });
    v.insert("goto_statement".to_string(), ContextId { index: 7345 });
    v.insert("nullify_statement".to_string(), ContextId { index: 7365 });
    v.insert("string_double".to_string(), ContextId { index: 7384 });
    v.insert("assignment_statement".to_string(), ContextId { index: 7305 });
    v.insert("other".to_string(), ContextId { index: 7368 });
    v.insert("square-braces".to_string(), ContextId { index: 7381 });
    v.insert("bind".to_string(), ContextId { index: 7307 });
    v.insert("#anon_declaration_type_spec_3".to_string(), ContextId { index: 7282 });
    v.insert("end_construct".to_string(), ContextId { index: 7331 });
    v.insert("else_statement".to_string(), ContextId { index: 7330 });
    v.insert("#anon_pop-at-end-of-line_0".to_string(), ContextId { index: 7294 });
    v.insert("case_selector".to_string(), ContextId { index: 7312 });
    v.insert("contains_statement".to_string(), ContextId { index: 7319 });
    v.insert("import_statement".to_string(), ContextId { index: 7352 });
    v.insert("operator".to_string(), ContextId { index: 7366 });
    v.insert("implicit_statement".to_string(), ContextId { index: 7351 });
    v.insert("function-arguments".to_string(), ContextId { index: 7342 });
    v.insert("entry_statement".to_string(), ContextId { index: 7333 });
    v.insert("pop-at-closing-parenthesis".to_string(), ContextId { index: 7369 });
    v.insert("#anon_print_statement_0".to_string(), ContextId { index: 7295 });
    v.insert("declaration_type_spec".to_string(), ContextId { index: 7324 });
    v.insert("if_block_statement".to_string(), ContextId { index: 7348 });
    v.insert("block_data_statement".to_string(), ContextId { index: 7308 });
    v.insert("format_string_double_quotes".to_string(), ContextId { index: 7339 });
    v.insert("coarrays".to_string(), ContextId { index: 7314 });
    v.insert("format_string".to_string(), ContextId { index: 7338 });
    v.insert("io_statements".to_string(), ContextId { index: 7357 });
    v.insert("associate_statement".to_string(), ContextId { index: 7306 });
    v.insert("dummy_argument_list".to_string(), ContextId { index: 7327 });
    v.insert("function-call".to_string(), ContextId { index: 7343 });
    v.insert("#anon_write_statement_1".to_string(), ContextId { index: 7299 });
    v.insert("comment".to_string(), ContextId { index: 7315 });
    v.insert("function-name".to_string(), ContextId { index: 7344 });
    v.insert("continue_statement".to_string(), ContextId { index: 7320 });
    v.insert("pop-at-end-of-line".to_string(), ContextId { index: 7370 });
    v.insert("misc_flow_control".to_string(), ContextId { index: 7362 });
    v.insert("__main".to_string(), ContextId { index: 7301 });
    v.insert("block_statement".to_string(), ContextId { index: 7309 });
    v.insert("comment_inline_latex".to_string(), ContextId { index: 7316 });
    v.insert("#anon_write_statement_2".to_string(), ContextId { index: 7300 });
    v.insert("#anon_declaration_type_spec_2".to_string(), ContextId { index: 7281 });
    v.insert("#anon_case_selector_0".to_string(), ContextId { index: 7278 });
    v.insert("#anon_end_program_unit_0".to_string(), ContextId { index: 7287 });
    v.insert("data_type_attributes".to_string(), ContextId { index: 7323 });
    v.insert("round-braces".to_string(), ContextId { index: 7379 });
    v.insert("else_if_statement".to_string(), ContextId { index: 7328 });
    v.insert("string_single".to_string(), ContextId { index: 7385 });
    v.insert("#anon_entry_statement_0".to_string(), ContextId { index: 7290 });
    v
  }
} }
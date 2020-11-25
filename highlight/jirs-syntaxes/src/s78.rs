
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Bourne Again Shell (bash)".to_string(),
  file_extensions: vec!["sh".to_string(),"bash".to_string(),"zsh".to_string(),"ash".to_string(),".bash_aliases".to_string(),".bash_completions".to_string(),".bash_functions".to_string(),".bash_login".to_string(),".bash_logout".to_string(),".bash_profile".to_string(),".bash_variables".to_string(),".bashrc".to_string(),".profile".to_string(),".textmate_init".to_string(),".zlogin".to_string(),".zlogout".to_string(),".zprofile".to_string(),".zshenv".to_string(),".zshrc".to_string(),"PKGBUILD".to_string(),".ebuild".to_string(),".eclass".to_string()],
  scope: Scope { a: 844742762627072, b: 0 },
  first_line_match: Some("(?x)\n  ^\\#! .* \\b(bash|zsh|sh|tcsh|ash)\\b\n| ^\\# \\s* -\\*- [^*]* mode: \\s* shell-script [^*]* -\\*-\n".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("identifier_non_posix".to_string(), "[^{{metachar}}\\d][^{{metachar}}=]*".to_string());
    v.insert("cmd_boundary".to_string(), "(?=\\s|;|$|>|<)".to_string());
    v.insert("is_command".to_string(), "(?=\\S)".to_string());
    v.insert("start_of_option".to_string(), "(?:\\s+|^)--?(?=[\\w$])".to_string());
    v.insert("varassign".to_string(), "[+\\-?]?=".to_string());
    v.insert("is_function".to_string(), "\\s*\\b(function)\\s+|(?=\\s*{{identifier_non_posix}}\\s*\\(\\s*\\))".to_string());
    v.insert("is_start_of_arguments".to_string(), "[`=|&;()<>\\s]".to_string());
    v.insert("is_end_of_interpolation".to_string(), "\\)".to_string());
    v.insert("is_end_of_option".to_string(), "[^\\w$-]|$".to_string());
    v.insert("is_variable".to_string(), "(?=\\s*{{nbc}}(?:[({]{{nbc}}[)}])?{{nbc}}=)".to_string());
    v.insert("keyword_break".to_string(), "(?![-=\\w])".to_string());
    v.insert("metachar".to_string(), "[\\s\\t\\n|&;()<>]".to_string());
    v.insert("nbc".to_string(), "[^{}()=\\s]*".to_string());
    v.insert("is_path_component".to_string(), "(?=[^\\s/]*/)".to_string());
    v.insert("call_token".to_string(), "\\./".to_string());
    v.insert("extension".to_string(), "\\.sh".to_string());
    v.insert("identifier".to_string(), "[[:alpha:]_][[:alnum:]_]*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("redirection-post".to_string(), ContextId { index: 5442 });
    v.insert("vardef-readonly-options".to_string(), ContextId { index: 5460 });
    v.insert("#anon_cmd-args-boilerplate-bt_0".to_string(), ContextId { index: 5311 });
    v.insert("#anon_cmd-test-brace-args_0".to_string(), ContextId { index: 5325 });
    v.insert("#anon_expansion-command_1".to_string(), ContextId { index: 5336 });
    v.insert("vardef-alias-name".to_string(), ContextId { index: 5452 });
    v.insert("cmd-test-brace-args".to_string(), ContextId { index: 5392 });
    v.insert("#anon_expression_2".to_string(), ContextId { index: 5351 });
    v.insert("#anon_string-quoted-double-escape-character_0".to_string(), ContextId { index: 5360 });
    v.insert("#anon_expansion-brace_0".to_string(), ContextId { index: 5334 });
    v.insert("case-clause-patterns-body".to_string(), ContextId { index: 5377 });
    v.insert("#anon_string-locale_0".to_string(), ContextId { index: 5359 });
    v.insert("expansion-arithmetic".to_string(), ContextId { index: 5402 });
    v.insert("case-word".to_string(), ContextId { index: 5379 });
    v.insert("operator-exclamation".to_string(), ContextId { index: 5433 });
    v.insert("#anon_expansion-parameter-post-first-character_6".to_string(), ContextId { index: 5343 });
    v.insert("#anon_expansion-parameter-post-first-character_1".to_string(), ContextId { index: 5338 });
    v.insert("cmd-test-brace-args-bt".to_string(), ContextId { index: 5393 });
    v.insert("string-ansi-c".to_string(), ContextId { index: 5445 });
    v.insert("#anon_expansion-command_0".to_string(), ContextId { index: 5335 });
    v.insert("#anon_line-continuation_0".to_string(), ContextId { index: 5355 });
    v.insert("heredocs-body".to_string(), ContextId { index: 5422 });
    v.insert("#anon_cmd-args_0".to_string(), ContextId { index: 5316 });
    v.insert("#anon_funcdef-name_0".to_string(), ContextId { index: 5353 });
    v.insert("arithmetic".to_string(), ContextId { index: 5372 });
    v.insert("comment".to_string(), ContextId { index: 5396 });
    v.insert("expansion-parameter-post-first-character".to_string(), ContextId { index: 5409 });
    v.insert("string-quoted-double-escape-character".to_string(), ContextId { index: 5449 });
    v.insert("#anon_redirection-process_0".to_string(), ContextId { index: 5357 });
    v.insert("any-escape".to_string(), ContextId { index: 5371 });
    v.insert("#anon_expansion-parameter-post-first-character_2".to_string(), ContextId { index: 5339 });
    v.insert("#anon_cmd-name_0".to_string(), ContextId { index: 5323 });
    v.insert("#anon_expansion-arithmetic_0".to_string(), ContextId { index: 5333 });
    v.insert("expansion-parameter-pattern".to_string(), ContextId { index: 5408 });
    v.insert("string-locale".to_string(), ContextId { index: 5446 });
    v.insert("__main".to_string(), ContextId { index: 5369 });
    v.insert("#anon_cmd-bt_0".to_string(), ContextId { index: 5319 });
    v.insert("#anon_vardef-value_1".to_string(), ContextId { index: 5366 });
    v.insert("control".to_string(), ContextId { index: 5397 });
    v.insert("redirection-here-document".to_string(), ContextId { index: 5437 });
    v.insert("#anon_cmd-args-bt_1".to_string(), ContextId { index: 5314 });
    v.insert("string-quoted-single".to_string(), ContextId { index: 5450 });
    v.insert("#anon_cmd-args-boilerplate_0".to_string(), ContextId { index: 5312 });
    v.insert("#anon_array_0".to_string(), ContextId { index: 5308 });
    v.insert("redirection".to_string(), ContextId { index: 5436 });
    v.insert("expansion-pattern".to_string(), ContextId { index: 5410 });
    v.insert("main".to_string(), ContextId { index: 5430 });
    v.insert("string-quoted-double".to_string(), ContextId { index: 5447 });
    v.insert("#anon_cmd-name-bt_0".to_string(), ContextId { index: 5322 });
    v.insert("case-end-ahead".to_string(), ContextId { index: 5378 });
    v.insert("expansion".to_string(), ContextId { index: 5400 });
    v.insert("expansion-brace".to_string(), ContextId { index: 5403 });
    v.insert("#anon_cmd-args_2".to_string(), ContextId { index: 5318 });
    v.insert("expansion-job".to_string(), ContextId { index: 5405 });
    v.insert("vardef-alias-options".to_string(), ContextId { index: 5453 });
    v.insert("cmd-post".to_string(), ContextId { index: 5391 });
    v.insert("redirection-process".to_string(), ContextId { index: 5443 });
    v.insert("#anon_case-clause-patterns-body_0".to_string(), ContextId { index: 5310 });
    v.insert("cmd-common".to_string(), ContextId { index: 5387 });
    v.insert("#anon_cmd_1".to_string(), ContextId { index: 5327 });
    v.insert("#anon_string-ansi-c_0".to_string(), ContextId { index: 5358 });
    v.insert("#anon_expansion-parameter-post-first-character_5".to_string(), ContextId { index: 5342 });
    v.insert("#anon_vardef-alias-name_0".to_string(), ContextId { index: 5363 });
    v.insert("cmd".to_string(), ContextId { index: 5380 });
    v.insert("expression".to_string(), ContextId { index: 5413 });
    v.insert("case-body".to_string(), ContextId { index: 5374 });
    v.insert("case-clause-patterns".to_string(), ContextId { index: 5376 });
    v.insert("funcdef".to_string(), ContextId { index: 5416 });
    v.insert("line-continuation".to_string(), ContextId { index: 5428 });
    v.insert("expansion-and-string".to_string(), ContextId { index: 5401 });
    v.insert("cmd-args-boilerplate-bt".to_string(), ContextId { index: 5383 });
    v.insert("cmd-name".to_string(), ContextId { index: 5388 });
    v.insert("#anon_vardef-value_2".to_string(), ContextId { index: 5367 });
    v.insert("#anon_vardef-value_0".to_string(), ContextId { index: 5365 });
    v.insert("#anon_cmd_2".to_string(), ContextId { index: 5328 });
    v.insert("heredocs-preamble".to_string(), ContextId { index: 5427 });
    v.insert("#anon_comment_0".to_string(), ContextId { index: 5329 });
    v.insert("vardef-maybe-more".to_string(), ContextId { index: 5458 });
    v.insert("main-with-pop-at-end".to_string(), ContextId { index: 5432 });
    v.insert("#anon_expansion-parameter-post-first-character_7".to_string(), ContextId { index: 5344 });
    v.insert("#anon_for-args_0".to_string(), ContextId { index: 5352 });
    v.insert("cmd-args-common".to_string(), ContextId { index: 5385 });
    v.insert("funcdef-body-bt".to_string(), ContextId { index: 5418 });
    v.insert("#anon_cmd-bt_2".to_string(), ContextId { index: 5321 });
    v.insert("expansion-tilde".to_string(), ContextId { index: 5412 });
    v.insert("cmd-name-common".to_string(), ContextId { index: 5390 });
    v.insert("heredocs-body-no-expansion".to_string(), ContextId { index: 5426 });
    v.insert("funcdef-parens".to_string(), ContextId { index: 5421 });
    v.insert("#anon_vardef-value_3".to_string(), ContextId { index: 5368 });
    v.insert("#anon_string-quoted-double_0".to_string(), ContextId { index: 5361 });
    v.insert("vardef".to_string(), ContextId { index: 5451 });
    v.insert("heredocs-body-allow-tabs".to_string(), ContextId { index: 5423 });
    v.insert("#anon_coproc-body_2".to_string(), ContextId { index: 5332 });
    v.insert("vardef-ensure-function-call-scope".to_string(), ContextId { index: 5456 });
    v.insert("redirection-output".to_string(), ContextId { index: 5441 });
    v.insert("vardef-export-options".to_string(), ContextId { index: 5457 });
    v.insert("#anon_case-body_0".to_string(), ContextId { index: 5309 });
    v.insert("pop-at-end".to_string(), ContextId { index: 5434 });
    v.insert("redirection-inout".to_string(), ContextId { index: 5439 });
    v.insert("expansion-parameter".to_string(), ContextId { index: 5406 });
    v.insert("string".to_string(), ContextId { index: 5444 });
    v.insert("#anon_expansion-parameter-post-first-character_4".to_string(), ContextId { index: 5341 });
    v.insert("coproc-body".to_string(), ContextId { index: 5398 });
    v.insert("#anon_cmd-args-bt_0".to_string(), ContextId { index: 5313 });
    v.insert("end-of-options-common".to_string(), ContextId { index: 5399 });
    v.insert("expansion-command".to_string(), ContextId { index: 5404 });
    v.insert("#anon_expression_1".to_string(), ContextId { index: 5350 });
    v.insert("cmd-test-double-brace-args-bt".to_string(), ContextId { index: 5395 });
    v.insert("#anon_cmd-bt_1".to_string(), ContextId { index: 5320 });
    v.insert("cmd-args".to_string(), ContextId { index: 5381 });
    v.insert("#anon_coproc-body_0".to_string(), ContextId { index: 5330 });
    v.insert("case-clause-commands".to_string(), ContextId { index: 5375 });
    v.insert("#anon_expansion-parameter-post-first-character_0".to_string(), ContextId { index: 5337 });
    v.insert("__start".to_string(), ContextId { index: 5370 });
    v.insert("#anon_cmd-args_1".to_string(), ContextId { index: 5317 });
    v.insert("#anon_cmd_0".to_string(), ContextId { index: 5326 });
    v.insert("cmd-name-bt".to_string(), ContextId { index: 5389 });
    v.insert("#anon_string-quoted-single_0".to_string(), ContextId { index: 5362 });
    v.insert("#anon_expansion-pattern_1".to_string(), ContextId { index: 5347 });
    v.insert("for-args".to_string(), ContextId { index: 5415 });
    v.insert("heredocs-body-common-with-expansion".to_string(), ContextId { index: 5425 });
    v.insert("main-bt".to_string(), ContextId { index: 5431 });
    v.insert("#anon_cmd-args-bt_2".to_string(), ContextId { index: 5315 });
    v.insert("cmd-bt".to_string(), ContextId { index: 5386 });
    v.insert("expansion-parameter-common".to_string(), ContextId { index: 5407 });
    v.insert("funcdef-body".to_string(), ContextId { index: 5417 });
    v.insert("line-continuation-or-pop-at-end".to_string(), ContextId { index: 5429 });
    v.insert("#anon_expression-test_0".to_string(), ContextId { index: 5348 });
    v.insert("funcdef-bt".to_string(), ContextId { index: 5419 });
    v.insert("#anon_vardef-name_0".to_string(), ContextId { index: 5364 });
    v.insert("#anon_coproc-body_1".to_string(), ContextId { index: 5331 });
    v.insert("#anon_arithmetic_0".to_string(), ContextId { index: 5307 });
    v.insert("#anon_expansion-parameter-post-first-character_3".to_string(), ContextId { index: 5340 });
    v.insert("array".to_string(), ContextId { index: 5373 });
    v.insert("#anon_expansion-parameter_0".to_string(), ContextId { index: 5345 });
    v.insert("expression-test".to_string(), ContextId { index: 5414 });
    v.insert("redirection-here-string".to_string(), ContextId { index: 5438 });
    v.insert("redirection-input".to_string(), ContextId { index: 5440 });
    v.insert("vardef-assign".to_string(), ContextId { index: 5454 });
    v.insert("vardef-value".to_string(), ContextId { index: 5461 });
    v.insert("prototype".to_string(), ContextId { index: 5435 });
    v.insert("#anon_expansion-pattern_0".to_string(), ContextId { index: 5346 });
    v.insert("string-quoted-double-common".to_string(), ContextId { index: 5448 });
    v.insert("cmd-args-bt".to_string(), ContextId { index: 5384 });
    v.insert("cmd-test-double-brace-args".to_string(), ContextId { index: 5394 });
    v.insert("#anon_expression_0".to_string(), ContextId { index: 5349 });
    v.insert("#anon_cmd-test-brace-args-bt_0".to_string(), ContextId { index: 5324 });
    v.insert("#anon_heredocs-preamble_0".to_string(), ContextId { index: 5354 });
    v.insert("#anon_redirection-post_0".to_string(), ContextId { index: 5356 });
    v.insert("cmd-args-boilerplate".to_string(), ContextId { index: 5382 });
    v.insert("expansion-pattern-post-first-char".to_string(), ContextId { index: 5411 });
    v.insert("heredocs-body-allow-tabs-no-expansion".to_string(), ContextId { index: 5424 });
    v.insert("vardef-declare-options".to_string(), ContextId { index: 5455 });
    v.insert("funcdef-name".to_string(), ContextId { index: 5420 });
    v.insert("vardef-name".to_string(), ContextId { index: 5459 });
    v
  }
} }

#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "OCaml".to_string(),
  file_extensions: vec!["ml".to_string(),"mli".to_string()],
  scope: Scope { a: 844648268431360, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("oct_digit".to_string(), "[0-7_]".to_string());
    v.insert("dec_digit".to_string(), "[0-9_]".to_string());
    v.insert("hex_exponent".to_string(), "(?:[Pp]{{exponent}})".to_string());
    v.insert("bin_integer".to_string(), "(?:[01]{{bin_digit}}*)".to_string());
    v.insert("hex_digit".to_string(), "[\\h_]".to_string());
    v.insert("identifier".to_string(), "[a-z][a-zA-Z0-9\'_]*".to_string());
    v.insert("bin_digit".to_string(), "[01_]".to_string());
    v.insert("dec_exponent".to_string(), "(?:[Ee]{{exponent}})".to_string());
    v.insert("exponent".to_string(), "[-+]??(_?){{dec_integer}}".to_string());
    v.insert("suffix".to_string(), "(?:[lLn]|(?!\\.))\\b".to_string());
    v.insert("capIdentifier".to_string(), "[A-Z][a-zA-Z0-9\'_]*".to_string());
    v.insert("dec_integer".to_string(), "(?:[0-9]{{dec_digit}}*)".to_string());
    v.insert("sign".to_string(), "(?:-|\\b)".to_string());
    v.insert("oct_integer".to_string(), "(?:[0-7]{{oct_digit}}*)".to_string());
    v.insert("hex_integer".to_string(), "(?:\\h{{hex_digit}}*)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_variables_1".to_string(), ContextId { index: 3499 });
    v.insert("strings".to_string(), ContextId { index: 3519 });
    v.insert("comments".to_string(), ContextId { index: 3510 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 3488 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 3470 });
    v.insert("#anon_main_9".to_string(), ContextId { index: 3489 });
    v.insert("matchpatterns".to_string(), ContextId { index: 3515 });
    v.insert("#anon_variables_7".to_string(), ContextId { index: 3505 });
    v.insert("arrays".to_string(), ContextId { index: 3509 });
    v.insert("moduleref".to_string(), ContextId { index: 3517 });
    v.insert("#anon_variables_2".to_string(), ContextId { index: 3500 });
    v.insert("#anon_main_15".to_string(), ContextId { index: 3475 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 3473 });
    v.insert("__start".to_string(), ContextId { index: 3508 });
    v.insert("#anon_module-signature_0".to_string(), ContextId { index: 3491 });
    v.insert("#anon_variables_3".to_string(), ContextId { index: 3501 });
    v.insert("#anon_matchpatterns_0".to_string(), ContextId { index: 3490 });
    v.insert("#anon_main_18".to_string(), ContextId { index: 3478 });
    v.insert("#anon_lists_0".to_string(), ContextId { index: 3467 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 3485 });
    v.insert("#anon_variables_0".to_string(), ContextId { index: 3498 });
    v.insert("typedefs".to_string(), ContextId { index: 3520 });
    v.insert("storagetypes".to_string(), ContextId { index: 3518 });
    v.insert("lists".to_string(), ContextId { index: 3513 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 3468 });
    v.insert("__main".to_string(), ContextId { index: 3507 });
    v.insert("#anon_main_16".to_string(), ContextId { index: 3476 });
    v.insert("#anon_variables_8".to_string(), ContextId { index: 3506 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 3495 });
    v.insert("#anon_main_21".to_string(), ContextId { index: 3482 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 3471 });
    v.insert("#anon_main_19".to_string(), ContextId { index: 3479 });
    v.insert("module-signature".to_string(), ContextId { index: 3516 });
    v.insert("definite_storagetypes".to_string(), ContextId { index: 3512 });
    v.insert("#anon_main_17".to_string(), ContextId { index: 3477 });
    v.insert("variables".to_string(), ContextId { index: 3521 });
    v.insert("#anon_arrays_0".to_string(), ContextId { index: 3464 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 3483 });
    v.insert("main".to_string(), ContextId { index: 3514 });
    v.insert("#anon_variables_6".to_string(), ContextId { index: 3504 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 3486 });
    v.insert("#anon_module-signature_1".to_string(), ContextId { index: 3492 });
    v.insert("#anon_module-signature_3".to_string(), ContextId { index: 3494 });
    v.insert("#anon_main_20".to_string(), ContextId { index: 3481 });
    v.insert("#anon_main_14".to_string(), ContextId { index: 3474 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 3487 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 3466 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 3469 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 3480 });
    v.insert("#anon_typedefs_0".to_string(), ContextId { index: 3496 });
    v.insert("#anon_typedefs_1".to_string(), ContextId { index: 3497 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 3465 });
    v.insert("#anon_variables_5".to_string(), ContextId { index: 3503 });
    v.insert("#anon_module-signature_2".to_string(), ContextId { index: 3493 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 3484 });
    v.insert("#anon_variables_4".to_string(), ContextId { index: 3502 });
    v.insert("constants".to_string(), ContextId { index: 3511 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 3472 });
    v
  }
} }

#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "MATLAB".to_string(),
  file_extensions: vec!["matlab".to_string()],
  scope: Scope { a: 844643973464064, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("id".to_string(), "[A-Za-z_]\\w*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("matlab_support_function".to_string(), ContextId { index: 3412 });
    v.insert("matlab_support_toolbox_financial_derivatives".to_string(), ContextId { index: 3426 });
    v.insert("matlab_support_toolbox_filter_design_hdl_coder".to_string(), ContextId { index: 3424 });
    v.insert("matlab_oop".to_string(), ContextId { index: 3407 });
    v.insert("matlab_support_graphics".to_string(), ContextId { index: 3413 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 3377 });
    v.insert("#anon_matlab_oop_0".to_string(), ContextId { index: 3382 });
    v.insert("#anon_matlab_oop_1".to_string(), ContextId { index: 3383 });
    v.insert("matlab_keyword_analysis".to_string(), ContextId { index: 3401 });
    v.insert("matlab_support_toolbox_opc".to_string(), ContextId { index: 3439 });
    v.insert("matlab_support_toolbox_signal_processing".to_string(), ContextId { index: 3443 });
    v.insert("curlybrackets".to_string(), ContextId { index: 3395 });
    v.insert("all_matlab_comments".to_string(), ContextId { index: 3390 });
    v.insert("main".to_string(), ContextId { index: 3399 });
    v.insert("#anon_curlybrackets_0".to_string(), ContextId { index: 3378 });
    v.insert("matlab_support_toolbox_curve_fitting".to_string(), ContextId { index: 3418 });
    v.insert("#anon_all_matlab_comments_0".to_string(), ContextId { index: 3376 });
    v.insert("matlab_support_toolbox_virtual_reality".to_string(), ContextId { index: 3448 });
    v.insert("#anon_function_1".to_string(), ContextId { index: 3380 });
    v.insert("#anon_matlab_oop_2".to_string(), ContextId { index: 3384 });
    v.insert("matlab_support_toolbox_image_acquisition".to_string(), ContextId { index: 3432 });
    v.insert("matlab_support_toolbox_robust_control".to_string(), ContextId { index: 3442 });
    v.insert("parens".to_string(), ContextId { index: 3455 });
    v.insert("matlab_support_toolbox_excel_link".to_string(), ContextId { index: 3423 });
    v.insert("matlab_support_toolbox_symbolic_math".to_string(), ContextId { index: 3446 });
    v.insert("matlab_storage_control".to_string(), ContextId { index: 3408 });
    v.insert("matlab_constant_language".to_string(), ContextId { index: 3400 });
    v.insert("transpose_post_parens".to_string(), ContextId { index: 3459 });
    v.insert("matlab_support_toolbox_model_predictive_control".to_string(), ContextId { index: 3437 });
    v.insert("matlab_support_toolbox_data_acquisition".to_string(), ContextId { index: 3419 });
    v.insert("matlab_support_toolbox_genetic_algorithms".to_string(), ContextId { index: 3431 });
    v.insert("matlab_support_toolbox_fixed_point".to_string(), ContextId { index: 3428 });
    v.insert("special_characters".to_string(), ContextId { index: 3456 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 3385 });
    v.insert("string".to_string(), ContextId { index: 3457 });
    v.insert("matlab_support_toolbox_statistics".to_string(), ContextId { index: 3445 });
    v.insert("matlab_storage_type".to_string(), ContextId { index: 3410 });
    v.insert("matlab_support_external".to_string(), ContextId { index: 3411 });
    v.insert("transpose".to_string(), ContextId { index: 3458 });
    v.insert("matlab_support_toolbox_fixed_income".to_string(), ContextId { index: 3427 });
    v.insert("matlab_support_toolbox_wavelet".to_string(), ContextId { index: 3449 });
    v.insert("__start".to_string(), ContextId { index: 3389 });
    v.insert("matlab_support_toolbox_bioinformatics".to_string(), ContextId { index: 3415 });
    v.insert("matlab_support_toolbox_datafeed".to_string(), ContextId { index: 3421 });
    v.insert("matlab_support_toolbox_neural_network".to_string(), ContextId { index: 3438 });
    v.insert("variable_invalid".to_string(), ContextId { index: 3463 });
    v.insert("members".to_string(), ContextId { index: 3451 });
    v.insert("unescaped_quote".to_string(), ContextId { index: 3460 });
    v.insert("variable_assignment".to_string(), ContextId { index: 3462 });
    v.insert("matlab_support_toolbox_mapping".to_string(), ContextId { index: 3435 });
    v.insert("#anon_function_0".to_string(), ContextId { index: 3379 });
    v.insert("matlab_keyword_desktop".to_string(), ContextId { index: 3403 });
    v.insert("matlab_keyword_operator".to_string(), ContextId { index: 3405 });
    v.insert("matlab_support_toolbox_aerospace".to_string(), ContextId { index: 3414 });
    v.insert("operators".to_string(), ContextId { index: 3454 });
    v.insert("matlab_support_toolbox_system_identification".to_string(), ContextId { index: 3447 });
    v.insert("end_in_parens".to_string(), ContextId { index: 3396 });
    v.insert("matlab_support_toolbox_image_processing".to_string(), ContextId { index: 3433 });
    v.insert("matlab_keyword_other".to_string(), ContextId { index: 3406 });
    v.insert("matlab_support_toolbox_optimization".to_string(), ContextId { index: 3440 });
    v.insert("matlab_keyword_mathematics".to_string(), ContextId { index: 3404 });
    v.insert("matlab_support_toolbox_communications".to_string(), ContextId { index: 3416 });
    v.insert("__main".to_string(), ContextId { index: 3388 });
    v.insert("matlab_support_toolbox_rf".to_string(), ContextId { index: 3441 });
    v.insert("all_matlab_keywords".to_string(), ContextId { index: 3391 });
    v.insert("matlab_support_toolbox_design".to_string(), ContextId { index: 3422 });
    v.insert("matlab_support_toolbox_fuzzy_logic".to_string(), ContextId { index: 3429 });
    v.insert("matlab_storage_modifier".to_string(), ContextId { index: 3409 });
    v.insert("matlab_support_toolbox_spline".to_string(), ContextId { index: 3444 });
    v.insert("matlab_variable_function".to_string(), ContextId { index: 3450 });
    v.insert("variable".to_string(), ContextId { index: 3461 });
    v.insert("matlab_support_toolbox_control_systems".to_string(), ContextId { index: 3417 });
    v.insert("#anon_string_1".to_string(), ContextId { index: 3387 });
    v.insert("escaped_quote".to_string(), ContextId { index: 3397 });
    v.insert("function".to_string(), ContextId { index: 3398 });
    v.insert("matlab_support_toolbox_garch".to_string(), ContextId { index: 3430 });
    v.insert("constants_override".to_string(), ContextId { index: 3394 });
    v.insert("#anon_function_2".to_string(), ContextId { index: 3381 });
    v.insert("matlab_support_toolbox_model_based_calibration".to_string(), ContextId { index: 3436 });
    v.insert("brackets".to_string(), ContextId { index: 3393 });
    v.insert("allofem".to_string(), ContextId { index: 3392 });
    v.insert("matlab_support_toolbox_financial".to_string(), ContextId { index: 3425 });
    v.insert("number".to_string(), ContextId { index: 3453 });
    v.insert("matlab_support_toolbox_database".to_string(), ContextId { index: 3420 });
    v.insert("matlab_support_toolbox_instrument_control".to_string(), ContextId { index: 3434 });
    v.insert("matlab_keyword_control".to_string(), ContextId { index: 3402 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 3386 });
    v.insert("not_equal_invalid".to_string(), ContextId { index: 3452 });
    v
  }
} }
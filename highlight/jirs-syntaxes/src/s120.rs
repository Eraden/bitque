
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "GLSL".to_string(),
  file_extensions: vec!["vs".to_string(),"fs".to_string(),"gs".to_string(),"vsh".to_string(),"fsh".to_string(),"gsh".to_string(),"vshader".to_string(),"fshader".to_string(),"gshader".to_string(),"vert".to_string(),"frag".to_string(),"geom".to_string(),"tesc".to_string(),"tese".to_string(),"comp".to_string(),"glsl".to_string()],
  scope: Scope { a: 844910261436416, b: 0 },
  first_line_match: Some("-[*]-( Mode:)? GLSL -[*]-".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("control_keywords".to_string(), "(?x) # ignore whitespace in this regex\n  break |\n  case |\n  continue |\n  default |\n  discard |\n  do |\n  else |\n  for |\n  if |\n  return |\n  switch |\n  while\n".to_string());
    v.insert("modifiers".to_string(), "(?x) # ignore whitespace in this regex\n  \\b(\n    layout |\n    attribute |\n    centroid |\n    sampler |\n    patch |\n    const |\n    flat |\n    in |\n    inout |\n    invariant |\n    noperspective |\n    out |\n    smooth |\n    uniform |\n    varying |\n    buffer |\n    shared |\n    coherent |\n    readonly |\n    writeonly |\n    volatile |\n    restrict\n  )\\b\n".to_string());
    v.insert("basic_types".to_string(), "(?x) # ignore whitespace in this regex\n  \\b(\n    void |\n    bool |\n    int |\n    uint |\n    float |\n    double |\n    vec[2-4] |\n    dvec[2-4] |\n    bvec[2-4] |\n    ivec[2-4] |\n    uvec[2-4] |\n    mat[2-4] |\n    mat2x2 |\n    mat2x3 |\n    mat2x4 |\n    mat3x2 |\n    mat3x3 |\n    mat3x4 |\n    mat4x2 |\n    mat4x3 |\n    mat4x4 |\n    dmat2 |\n    dmat3 |\n    dmat4 |\n    dmat2x2 |\n    dmat2x3 |\n    dmat2x4 |\n    dmat3x2 |\n    dmat3x3 |\n    dmat3x4 |\n    dmat4x2 |\n    dmat4x3 |\n    dmat4x4 |\n    sampler[1-3]D |\n    image[1-3]D |\n    samplerCube |\n    imageCube |\n    sampler2DRect |\n    image2DRect |\n    sampler[12]DArray |\n    image[12]DArray |\n    samplerBuffer |\n    imageBuffer |\n    sampler2DMS |\n    image2DMS |\n    sampler2DMSArray |\n    image2DMSArray |\n    samplerCubeArray |\n    imageCubeArray |\n    sampler[12]DShadow |\n    sampler2DRectShadow |\n    sampler[12]DArrayShadow |\n    samplerCubeShadow |\n    samplerCubeArrayShadow |\n    isampler[1-3]D |\n    iimage[1-3]D |\n    isamplerCube |\n    iimageCube |\n    isampler2DRect |\n    iimage2DRect |\n    isampler[12]DArray |\n    iimage[12]DArray |\n    isamplerBuffer |\n    iimageBuffer |\n    isampler2DMS |\n    iimage2DMS |\n    isampler2DMSArray |\n    iimage2DMSArray |\n    isamplerCubeArray |\n    iimageCubeArray |\n    atomic_uint |\n    usampler[1-3]D |\n    uimage[1-3]D |\n    usamplerCube |\n    uimageCube |\n    usampler2DRect |\n    uimage2DRect |\n    usampler[12]DArray |\n    uimage[12]DArray |\n    usamplerBuffer |\n    uimageBuffer |\n    usampler2DMS |\n    uimage2DMS |\n    usampler2DMSArray |\n    uimage2DMSArray |\n    usamplerCubeArray |\n    uimageCubeArray\n  )\\b\n".to_string());
    v.insert("non_func_keywords".to_string(), "if|for|switch|while".to_string());
    v.insert("before_tag".to_string(), "struct".to_string());
    v.insert("identifier".to_string(), "\\b[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v.insert("reserved_keyword_for_future_use".to_string(), "(?x) # ignore whitespace in this regex\n  \\b(?:\n    common |\n    partition |\n    active |\n    asm |\n    class |\n    union |\n    enum |\n    typedef |\n    template |\n    this |\n    resource |\n    goto |\n    inline |\n    noinline |\n    public |\n    static |\n    extern |\n    external |\n    interface |\n    long |\n    short |\n    half |\n    fixed |\n    unsigned |\n    superp |\n    input |\n    output |\n    hvec2 |\n    hvec3 |\n    hvec4 |\n    fvec2 |\n    fvec3 |\n    fvec4 |\n    sampler3DRect |\n    filter |\n    sizeof |\n    cast |\n    namespace |\n    using\n  )\\b\n".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("preprocessor-line-ending".to_string(), ContextId { index: 7555 });
    v.insert("preprocessor-block-finish-statements".to_string(), ContextId { index: 7539 });
    v.insert("#anon_preprocessor-rule-enabled-global_2".to_string(), ContextId { index: 7495 });
    v.insert("#anon_preprocessor-rule-enabled-statements_1".to_string(), ContextId { index: 7497 });
    v.insert("access".to_string(), ContextId { index: 7504 });
    v.insert("#anon_preprocessor-macro-extension-inside_0".to_string(), ContextId { index: 7473 });
    v.insert("#anon_preprocessor-macro-params_0".to_string(), ContextId { index: 7475 });
    v.insert("global-modifier".to_string(), ContextId { index: 7524 });
    v.insert("preprocessor-if-branch-function-call-arguments".to_string(), ContextId { index: 7550 });
    v.insert("preprocessor-rule-enabled-global".to_string(), ContextId { index: 7568 });
    v.insert("#anon_preprocessor-disabled_0".to_string(), ContextId { index: 7471 });
    v.insert("#anon_preprocessor-rule-disabled-statements_0".to_string(), ContextId { index: 7487 });
    v.insert("#anon_block_0".to_string(), ContextId { index: 7454 });
    v.insert("brackets".to_string(), ContextId { index: 7506 });
    v.insert("late-expressions".to_string(), ContextId { index: 7528 });
    v.insert("typedef".to_string(), ContextId { index: 7574 });
    v.insert("modifiers".to_string(), ContextId { index: 7530 });
    v.insert("#anon_preprocessor-other_0".to_string(), ContextId { index: 7476 });
    v.insert("#anon_preprocessor-other_2".to_string(), ContextId { index: 7478 });
    v.insert("preprocessor-if-branch-global".to_string(), ContextId { index: 7552 });
    v.insert("preprocessor-macro-definition".to_string(), ContextId { index: 7558 });
    v.insert("preprocessor-rule-disabled-global".to_string(), ContextId { index: 7565 });
    v.insert("preprocessor-rule-disabled-data-structures".to_string(), ContextId { index: 7564 });
    v.insert("data-structures-body".to_string(), ContextId { index: 7511 });
    v.insert("variables".to_string(), ContextId { index: 7576 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 7458 });
    v.insert("main".to_string(), ContextId { index: 7529 });
    v.insert("preprocessor-block-finish-global".to_string(), ContextId { index: 7536 });
    v.insert("preprocessor-elif-else-branch-global".to_string(), ContextId { index: 7545 });
    v.insert("#anon_preprocessor-other_1".to_string(), ContextId { index: 7477 });
    v.insert("#anon_preprocessor-rule-disabled-statements_1".to_string(), ContextId { index: 7488 });
    v.insert("preprocessor-macro-extension".to_string(), ContextId { index: 7559 });
    v.insert("global-maybe-function".to_string(), ContextId { index: 7523 });
    v.insert("preprocessor-macro-version".to_string(), ContextId { index: 7562 });
    v.insert("#anon_function-definition-body_0".to_string(), ContextId { index: 7462 });
    v.insert("function-call".to_string(), ContextId { index: 7517 });
    v.insert("#anon_preprocessor-rule-enabled-statements_0".to_string(), ContextId { index: 7496 });
    v.insert("#anon_preprocessor-rule-other-global_0".to_string(), ContextId { index: 7499 });
    v.insert("function-definition-body".to_string(), ContextId { index: 7518 });
    v.insert("parens".to_string(), ContextId { index: 7534 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 7468 });
    v.insert("preprocessor-block-if-branch-statements".to_string(), ContextId { index: 7541 });
    v.insert("preprocessor-macro-define".to_string(), ContextId { index: 7556 });
    v.insert("#anon_global-type_0".to_string(), ContextId { index: 7465 });
    v.insert("types".to_string(), ContextId { index: 7575 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_0".to_string(), ContextId { index: 7481 });
    v.insert("#anon_preprocessor-rule-disabled-global_1".to_string(), ContextId { index: 7485 });
    v.insert("#anon_preprocessor-comments_0".to_string(), ContextId { index: 7469 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_2".to_string(), ContextId { index: 7483 });
    v.insert("#anon_preprocessor-rule-enabled-global_0".to_string(), ContextId { index: 7493 });
    v.insert("#anon_preprocessor-rule-enabled-global_1".to_string(), ContextId { index: 7494 });
    v.insert("#anon_preprocessor-rule-other-statements_0".to_string(), ContextId { index: 7500 });
    v.insert("global-type".to_string(), ContextId { index: 7525 });
    v.insert("preprocessor-macro-define-or-version-inside".to_string(), ContextId { index: 7557 });
    v.insert("operators".to_string(), ContextId { index: 7533 });
    v.insert("statements".to_string(), ContextId { index: 7573 });
    v.insert("#anon_function-definition-params_1".to_string(), ContextId { index: 7464 });
    v.insert("preprocessor-rule-other-statements".to_string(), ContextId { index: 7571 });
    v.insert("numbers".to_string(), ContextId { index: 7532 });
    v.insert("preprocessor-rule-disabled-statements".to_string(), ContextId { index: 7566 });
    v.insert("preprocessor-if-branch-function-call".to_string(), ContextId { index: 7549 });
    v.insert("preprocessor-macro-extension-inside".to_string(), ContextId { index: 7560 });
    v.insert("functions-builtin".to_string(), ContextId { index: 7521 });
    v.insert("preprocessor-expressions".to_string(), ContextId { index: 7547 });
    v.insert("expressions".to_string(), ContextId { index: 7516 });
    v.insert("preprocessor-global".to_string(), ContextId { index: 7548 });
    v.insert("data-structures".to_string(), ContextId { index: 7510 });
    v.insert("data-structures-definition-common-begin".to_string(), ContextId { index: 7512 });
    v.insert("preprocessor-rule-enabled-statements".to_string(), ContextId { index: 7569 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_0".to_string(), ContextId { index: 7490 });
    v.insert("preprocessor-rule-other-global".to_string(), ContextId { index: 7570 });
    v.insert("#anon_preprocessor-rule-disabled-statements_2".to_string(), ContextId { index: 7489 });
    v.insert("preprocessor-block-finish-if-branch-global".to_string(), ContextId { index: 7537 });
    v.insert("#anon_preprocessor-macro-extension_0".to_string(), ContextId { index: 7474 });
    v.insert("preprocessor-disabled".to_string(), ContextId { index: 7544 });
    v.insert("#anon_preprocessor-rule-enabled-statements_2".to_string(), ContextId { index: 7498 });
    v.insert("#anon_preprocessor-other_4".to_string(), ContextId { index: 7480 });
    v.insert("#anon_negated-block_0".to_string(), ContextId { index: 7467 });
    v.insert("function-definition-continue".to_string(), ContextId { index: 7519 });
    v.insert("preprocessor-elif-else-branch-statements".to_string(), ContextId { index: 7546 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 7457 });
    v.insert("#anon_data-structures-struct-definition_0".to_string(), ContextId { index: 7459 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_1".to_string(), ContextId { index: 7491 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_1".to_string(), ContextId { index: 7482 });
    v.insert("comments".to_string(), ContextId { index: 7508 });
    v.insert("preprocessor-comments".to_string(), ContextId { index: 7542 });
    v.insert("preprocessor-if-branch-function-call-arguments-finish".to_string(), ContextId { index: 7551 });
    v.insert("preprocessor-rule-enabled-data-structures".to_string(), ContextId { index: 7567 });
    v.insert("preprocessor-statements".to_string(), ContextId { index: 7572 });
    v.insert("early-expressions".to_string(), ContextId { index: 7515 });
    v.insert("preprocessor-macro-params".to_string(), ContextId { index: 7561 });
    v.insert("#anon_preprocessor-other_3".to_string(), ContextId { index: 7479 });
    v.insert("preprocessor-if-branch-statements".to_string(), ContextId { index: 7553 });
    v.insert("data-structures-definition-common-end".to_string(), ContextId { index: 7513 });
    v.insert("constants".to_string(), ContextId { index: 7509 });
    v.insert("#anon_function-call_1".to_string(), ContextId { index: 7461 });
    v.insert("block".to_string(), ContextId { index: 7505 });
    v.insert("#anon_preprocessor-comments_1".to_string(), ContextId { index: 7470 });
    v.insert("preprocessor-data-structures".to_string(), ContextId { index: 7543 });
    v.insert("pragma-mark".to_string(), ContextId { index: 7535 });
    v.insert("#anon_preprocessor-rule-disabled-global_0".to_string(), ContextId { index: 7484 });
    v.insert("case-default".to_string(), ContextId { index: 7507 });
    v.insert("function-definition-params".to_string(), ContextId { index: 7520 });
    v.insert("__main".to_string(), ContextId { index: 7502 });
    v.insert("negated-block".to_string(), ContextId { index: 7531 });
    v.insert("#anon_preprocessor-macro-define-or-version-inside_0".to_string(), ContextId { index: 7472 });
    v.insert("__start".to_string(), ContextId { index: 7503 });
    v.insert("keywords".to_string(), ContextId { index: 7527 });
    v.insert("#anon_preprocessor-rule-disabled-global_2".to_string(), ContextId { index: 7486 });
    v.insert("preprocessor-line-continuation".to_string(), ContextId { index: 7554 });
    v.insert("global".to_string(), ContextId { index: 7522 });
    v.insert("#anon_typedef_0".to_string(), ContextId { index: 7501 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 7455 });
    v.insert("#anon_case-default_0".to_string(), ContextId { index: 7456 });
    v.insert("#anon_global-type_1".to_string(), ContextId { index: 7466 });
    v.insert("incomplete-inc".to_string(), ContextId { index: 7526 });
    v.insert("data-structures-struct-definition".to_string(), ContextId { index: 7514 });
    v.insert("preprocessor-block-finish-if-branch-statements".to_string(), ContextId { index: 7538 });
    v.insert("#anon_function-call_0".to_string(), ContextId { index: 7460 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_2".to_string(), ContextId { index: 7492 });
    v.insert("#anon_function-definition-params_0".to_string(), ContextId { index: 7463 });
    v.insert("preprocessor-other".to_string(), ContextId { index: 7563 });
    v.insert("preprocessor-block-if-branch-global".to_string(), ContextId { index: 7540 });
    v
  }
} }
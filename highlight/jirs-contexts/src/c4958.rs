
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
      regex: Regex::new("\\bcatch\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636706177426,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:fail|raise|throw)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701196709,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bloop\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636708536623,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(initialize|new|include|extend|prepend|attr_reader|attr_writer|attr_accessor|attr|module_function|public|protected|private)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636787102253122,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(require|require_relative|gem)\\b"),
      scope: vec![
        Scope {
            a: 52636636717449282,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4856 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    to_ary|\n    to_a|\n    to_c|\n    to_enum|\n    to_f|\n    to_hash|\n    to_h|\n    to_int|\n    to_io|\n    to_i|\n    to_proc|\n    to_r|\n    to_str|\n    to_sym|\n    to_s\n  )\\b\n  (?![!?=])\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    gsub|\n    sub\n  )(!|\\b)\n  |\n  \\b(\n    match\n  )(\\?|\\b)\n  |\n  \\b(\n    assert_match|\n    assert_no_match|\n    index|\n    rindex|\n    scan\n  )\\b(?![!?=])\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    eql\\?|\n    instance_of\\?|\n    instance_variable_defined\\?|\n    is_a\\?|\n    kind_of\\?|\n    nil\\?|\n    respond_to\\?|\n    respond_to_missing\\?|\n    tainted\\?|\n    untrusted\\?\n  )\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    class|\n    clone|\n    define_singleton_method|\n    display|\n    dup|\n    enum_for|\n    extend|\n    freeze|\n    frozen?|\n    hash|\n    inspect|\n    instance_variable_get|\n    instance_variable_set|\n    instance_variables|\n    itself|\n    method|\n    methods|\n    object_id|\n    private_methods|\n    protected_methods|\n    public_method|\n    public_methods|\n    public_send|\n    remove_instance_variable|\n    send|\n    singleton_class|\n    singleton_method|\n    singleton_methods|\n    taint|\n    tap|\n    trust|\n    untaint|\n    untrust\n  )\\b\n  (?![!?=])\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    autoload\\?|\n    iterator\\?|\n    exit!\n  )\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    Array|\n    Complex|\n    Float|\n    Hash|\n    Integer|\n    Rational|\n    String|\n    __callee__|\n    __dir__|\n    __method__|\n    abort|\n    at_exit|\n    autoload|\n    binding|\n    callcc|\n    caller|\n    caller_locations|\n    chomp|\n    chop|\n    eval|\n    exec|\n    exit|\n    fork|\n    format|\n    gets|\n    global_variables|\n    gsub|\n    lambda|\n    load|\n    local_variables|\n    open|\n    p|\n    print|\n    printf|\n    proc|\n    putc|\n    puts|\n    rand|\n    readline|\n    readlines|\n    require|\n    require_relative|\n    select|\n    set_trace_func|\n    sleep|\n    spawn|\n    sprintf|\n    srand|\n    sub|\n    syscall|\n    system|\n    test|\n    trace_var|\n    trap|\n    untrace_var|\n    warn\n  )\\b\n  (?![!?=])\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    class_variable_defined\\?|\n    const_defined\\?|\n    include\\?|\n    instance_methods\\?|\n    method_defined\\?|\n    private_method_defined\\?|\n    protected_method_defined\\?|\n    public_method_defined\\?|\n    singleton_class\\?\n  )\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  \\b(\n    ancestors|\n    append_features|\n    class_eval|\n    class_exec|\n    class_variable_get|\n    class_variable_set|\n    class_variables|\n    const_get|\n    const_missing|\n    const_set|\n    constants|\n    define_method|\n    extend_object|\n    extended|\n    freeze|\n    included|\n    included_modules|\n    inspect|\n    method_added|\n    method_removed|\n    method_undefined|\n    module_eval|\n    module_exec|\n    name|\n    prepend_features|\n    prepended|\n    private_class_method|\n    private_constant|\n    private_instance_methods|\n    protected_instance_methods|\n    public_class_method|\n    public_constant|\n    public_instance_method|\n    public_instance_methods|\n    refine|\n    remove_class_variable|\n    remove_const|\n    remove_method|\n    undef_method|\n    using\n  )\\b\n  (?![!?=])\n)"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4874 }),
    ]),
      with_prototype: None
    }),
]
} }
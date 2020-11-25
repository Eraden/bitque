
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Objective-C".to_string(),
  file_extensions: vec!["m".to_string(),"h".to_string()],
  scope: Scope { a: 844669743267840, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("non_func_keywords".to_string(), "if|for|switch|while|decltype|sizeof|__declspec|__attribute__".to_string());
    v.insert("common_protocols".to_string(), "\\bNS(GlyphStorage|M(utableCopying|enuItem)|C(hangeSpelling|o(ding|pying|lorPicking(Custom|Default)))|T(oolbarItemValidations|ext(Input|AttachmentCell))|I(nputServ(iceProvider|erMouseTracker)|gnoreMisspelledWords)|Obj(CTypeSerializationCallBack|ect)|D(ecimalNumberBehaviors|raggingInfo)|U(serInterfaceValidations|RL(HandleClient|DownloadDelegate|ProtocolClient|AuthenticationChallengeSender))|Validated(ToobarItem|UserInterfaceItem)|Locking)\\b".to_string());
    v.insert("before_tag".to_string(), "struct|union|enum".to_string());
    v.insert("identifier".to_string(), "\\b[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v.insert("control_keywords".to_string(), "break|case|continue|default|do|else|for|goto|if|_Pragma|return|switch|while".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("objc-structure-body".to_string(), ContextId { index: 3933 });
    v.insert("#anon_preprocessor-rule-enabled-objc-structures_1".to_string(), ContextId { index: 3875 });
    v.insert("data-structures-union-definition-after-name".to_string(), ContextId { index: 3910 });
    v.insert("numbers".to_string(), ContextId { index: 3932 });
    v.insert("global-maybe-function".to_string(), ContextId { index: 3919 });
    v.insert("preprocessor-macro-definition".to_string(), ContextId { index: 3954 });
    v.insert("ns-predicate".to_string(), ContextId { index: 3931 });
    v.insert("#anon_preprocessor-rule-other-statements_0".to_string(), ContextId { index: 3881 });
    v.insert("method".to_string(), ContextId { index: 3926 });
    v.insert("#anon_modifiers-parens_1".to_string(), ContextId { index: 3840 });
    v.insert("#anon_preprocessor-rule-disabled-global_2".to_string(), ContextId { index: 3861 });
    v.insert("#anon_function-call_1".to_string(), ContextId { index: 3827 });
    v.insert("preprocessor-block-finish-global".to_string(), ContextId { index: 3937 });
    v.insert("preprocessor-rule-enabled-global".to_string(), ContextId { index: 3964 });
    v.insert("preprocessor-block-if-branch-global".to_string(), ContextId { index: 3941 });
    v.insert("properties".to_string(), ContextId { index: 3970 });
    v.insert("access".to_string(), ContextId { index: 3891 });
    v.insert("preprocessor-block-finish-if-branch-global".to_string(), ContextId { index: 3938 });
    v.insert("#anon_preprocessor-rule-enabled-statements_2".to_string(), ContextId { index: 3879 });
    v.insert("preprocessor-rule-enabled-statements".to_string(), ContextId { index: 3966 });
    v.insert("comments".to_string(), ContextId { index: 3896 });
    v.insert("data-structures-definition-common-macro".to_string(), ContextId { index: 3902 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 3821 });
    v.insert("objc-structures".to_string(), ContextId { index: 3934 });
    v.insert("#anon_preprocessor-rule-enabled-objc-structures_0".to_string(), ContextId { index: 3874 });
    v.insert("preprocessor-rule-disabled-objc-structures".to_string(), ContextId { index: 3961 });
    v.insert("#anon_case-default_0".to_string(), ContextId { index: 3822 });
    v.insert("#anon_preprocessor-macro-define_1".to_string(), ContextId { index: 3848 });
    v.insert("#anon_keywords-parens_0".to_string(), ContextId { index: 3833 });
    v.insert("#anon_preprocessor-rule-enabled-objc-structures_2".to_string(), ContextId { index: 3876 });
    v.insert("ns-identifiers".to_string(), ContextId { index: 3930 });
    v.insert("preprocessor-rule-enabled-objc-structures".to_string(), ContextId { index: 3965 });
    v.insert("strings".to_string(), ContextId { index: 3974 });
    v.insert("data-structures-definition-common-begin".to_string(), ContextId { index: 3900 });
    v.insert("#anon_global-type_1".to_string(), ContextId { index: 3832 });
    v.insert("brackets".to_string(), ContextId { index: 3894 });
    v.insert("#anon_protocol_list_0".to_string(), ContextId { index: 3883 });
    v.insert("preprocessor-rule-disabled-statements".to_string(), ContextId { index: 3962 });
    v.insert("unique-modifiers".to_string(), ContextId { index: 3979 });
    v.insert("#anon_method_1".to_string(), ContextId { index: 3835 });
    v.insert("negated-block".to_string(), ContextId { index: 3929 });
    v.insert("preprocessor-statements".to_string(), ContextId { index: 3969 });
    v.insert("#anon_data-structures-enum-definition-block-start_0".to_string(), ContextId { index: 3823 });
    v.insert("#anon_unique-strings_0".to_string(), ContextId { index: 3885 });
    v.insert("preprocessor-elif-else-branch-statements".to_string(), ContextId { index: 3945 });
    v.insert("#anon_method_0".to_string(), ContextId { index: 3834 });
    v.insert("#anon_objc-structures_0".to_string(), ContextId { index: 3844 });
    v.insert("global-type".to_string(), ContextId { index: 3921 });
    v.insert("preprocessor-if-branch-statements".to_string(), ContextId { index: 3952 });
    v.insert("unique-strings".to_string(), ContextId { index: 3980 });
    v.insert("#anon_preprocessor-rule-disabled-statements_1".to_string(), ContextId { index: 3866 });
    v.insert("#anon_preprocessor-rule-disabled-objc-structures_1".to_string(), ContextId { index: 3863 });
    v.insert("operators".to_string(), ContextId { index: 3935 });
    v.insert("preprocessor-macro-define".to_string(), ContextId { index: 3953 });
    v.insert("#anon_global-type_0".to_string(), ContextId { index: 3831 });
    v.insert("data-structures-enum-definition-after-name".to_string(), ContextId { index: 3904 });
    v.insert("constants".to_string(), ContextId { index: 3897 });
    v.insert("typedef".to_string(), ContextId { index: 3975 });
    v.insert("preprocessor-elif-else-branch-global".to_string(), ContextId { index: 3944 });
    v.insert("#anon_preprocessor-rule-disabled-objc-structures_2".to_string(), ContextId { index: 3864 });
    v.insert("#anon_method_2".to_string(), ContextId { index: 3836 });
    v.insert("#anon_bracketed-content_1".to_string(), ContextId { index: 3819 });
    v.insert("#anon_function-call_0".to_string(), ContextId { index: 3826 });
    v.insert("#anon_unique-types_0".to_string(), ContextId { index: 3886 });
    v.insert("preprocessor-data-structures".to_string(), ContextId { index: 3943 });
    v.insert("preprocessor-if-branch-function-call-arguments".to_string(), ContextId { index: 3949 });
    v.insert("preprocessor-global".to_string(), ContextId { index: 3947 });
    v.insert("preprocessor-macro-params".to_string(), ContextId { index: 3955 });
    v.insert("#anon_negated-block_0".to_string(), ContextId { index: 3842 });
    v.insert("data-structures-body".to_string(), ContextId { index: 3899 });
    v.insert("data-structures".to_string(), ContextId { index: 3898 });
    v.insert("variables".to_string(), ContextId { index: 3983 });
    v.insert("#anon_preprocessor-other-include-common_0".to_string(), ContextId { index: 3850 });
    v.insert("#anon_function-definition-params_1".to_string(), ContextId { index: 3830 });
    v.insert("data-structures-struct-definition".to_string(), ContextId { index: 3906 });
    v.insert("#anon_modifiers-parens_2".to_string(), ContextId { index: 3841 });
    v.insert("#anon_preprocessor-other_3".to_string(), ContextId { index: 3855 });
    v.insert("#anon_preprocessor-macro-define_0".to_string(), ContextId { index: 3847 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_2".to_string(), ContextId { index: 3870 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_0".to_string(), ContextId { index: 3856 });
    v.insert("bracketed-content".to_string(), ContextId { index: 3893 });
    v.insert("case-default".to_string(), ContextId { index: 3895 });
    v.insert("#anon_modifiers-parens_0".to_string(), ContextId { index: 3839 });
    v.insert("#anon_method_3".to_string(), ContextId { index: 3837 });
    v.insert("function-definition-continue".to_string(), ContextId { index: 3916 });
    v.insert("__main".to_string(), ContextId { index: 3889 });
    v.insert("#anon_bracketed-content_0".to_string(), ContextId { index: 3818 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 3846 });
    v.insert("#anon_preprocessor-rule-enabled-global_1".to_string(), ContextId { index: 3872 });
    v.insert("global".to_string(), ContextId { index: 3918 });
    v.insert("global-modifier".to_string(), ContextId { index: 3920 });
    v.insert("#anon_typedef_0".to_string(), ContextId { index: 3884 });
    v.insert("keywords-parens".to_string(), ContextId { index: 3923 });
    v.insert("#anon_ns-predicate_0".to_string(), ContextId { index: 3843 });
    v.insert("preprocessor-block-finish-if-branch-statements".to_string(), ContextId { index: 3939 });
    v.insert("preprocessor-block-finish-statements".to_string(), ContextId { index: 3940 });
    v.insert("#anon_preprocessor-other_0".to_string(), ContextId { index: 3852 });
    v.insert("preprocessor-if-branch-function-call".to_string(), ContextId { index: 3948 });
    v.insert("unique-constants".to_string(), ContextId { index: 3977 });
    v.insert("preprocessor-rule-other-statements".to_string(), ContextId { index: 3968 });
    v.insert("#anon_function-definition-params_0".to_string(), ContextId { index: 3829 });
    v.insert("protocol_type_qualifier".to_string(), ContextId { index: 3972 });
    v.insert("protocol_list".to_string(), ContextId { index: 3971 });
    v.insert("#anon_preprocessor-rule-disabled-objc-structures_0".to_string(), ContextId { index: 3862 });
    v.insert("unique-types".to_string(), ContextId { index: 3981 });
    v.insert("#anon_preprocessor-other-include-common_1".to_string(), ContextId { index: 3851 });
    v.insert("#anon_function-definition-body_0".to_string(), ContextId { index: 3828 });
    v.insert("types".to_string(), ContextId { index: 3976 });
    v.insert("data-structures-enum-definition-block-start".to_string(), ContextId { index: 3905 });
    v.insert("#anon_data-structures-struct-definition-block-start_0".to_string(), ContextId { index: 3824 });
    v.insert("#anon_method_4".to_string(), ContextId { index: 3838 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_1".to_string(), ContextId { index: 3857 });
    v.insert("#anon_data-structures-union-definition-block-start_0".to_string(), ContextId { index: 3825 });
    v.insert("#anon_preprocessor-rule-disabled-global_1".to_string(), ContextId { index: 3860 });
    v.insert("#anon_bracketed-content_2".to_string(), ContextId { index: 3820 });
    v.insert("late-expressions".to_string(), ContextId { index: 3924 });
    v.insert("parens".to_string(), ContextId { index: 3936 });
    v.insert("preprocessor-other".to_string(), ContextId { index: 3957 });
    v.insert("modifiers-parens".to_string(), ContextId { index: 3928 });
    v.insert("function-definition-params".to_string(), ContextId { index: 3917 });
    v.insert("preprocessor-rule-disabled-data-structures".to_string(), ContextId { index: 3959 });
    v.insert("unique-variables".to_string(), ContextId { index: 3982 });
    v.insert("#anon_preprocessor-rule-disabled-global_0".to_string(), ContextId { index: 3859 });
    v.insert("#anon_preprocessor-rule-other-global_0".to_string(), ContextId { index: 3880 });
    v.insert("#anon_preprocessor-rule-disabled-statements_0".to_string(), ContextId { index: 3865 });
    v.insert("#anon_preprocessor-rule-enabled-global_2".to_string(), ContextId { index: 3873 });
    v.insert("data-structures-enum-definition".to_string(), ContextId { index: 3903 });
    v.insert("data-structures-struct-definition-after-name".to_string(), ContextId { index: 3907 });
    v.insert("main".to_string(), ContextId { index: 3925 });
    v.insert("#anon_block_0".to_string(), ContextId { index: 3817 });
    v.insert("#anon_unique-types_1".to_string(), ContextId { index: 3887 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_2".to_string(), ContextId { index: 3858 });
    v.insert("modifiers".to_string(), ContextId { index: 3927 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_1".to_string(), ContextId { index: 3869 });
    v.insert("#anon_preprocessor-rule-enabled-statements_0".to_string(), ContextId { index: 3877 });
    v.insert("data-structures-struct-definition-block-start".to_string(), ContextId { index: 3908 });
    v.insert("preprocessor-objc-structures".to_string(), ContextId { index: 3956 });
    v.insert("#anon_properties_0".to_string(), ContextId { index: 3882 });
    v.insert("statements".to_string(), ContextId { index: 3973 });
    v.insert("unique-keywords".to_string(), ContextId { index: 3978 });
    v.insert("preprocessor-rule-disabled-global".to_string(), ContextId { index: 3960 });
    v.insert("preprocessor-if-branch-function-call-arguments-finish".to_string(), ContextId { index: 3950 });
    v.insert("preprocessor-rule-other-global".to_string(), ContextId { index: 3967 });
    v.insert("#anon_preprocessor-rule-enabled-statements_1".to_string(), ContextId { index: 3878 });
    v.insert("preprocessor-rule-enabled-data-structures".to_string(), ContextId { index: 3963 });
    v.insert("#anon_preprocessor-other_2".to_string(), ContextId { index: 3854 });
    v.insert("#anon_preprocessor-rule-enabled-global_0".to_string(), ContextId { index: 3871 });
    v.insert("preprocessor-other-include-common".to_string(), ContextId { index: 3958 });
    v.insert("keywords".to_string(), ContextId { index: 3922 });
    v.insert("__start".to_string(), ContextId { index: 3890 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_0".to_string(), ContextId { index: 3868 });
    v.insert("preprocessor-expressions".to_string(), ContextId { index: 3946 });
    v.insert("function-call".to_string(), ContextId { index: 3914 });
    v.insert("#anon_preprocessor-macro-params_0".to_string(), ContextId { index: 3849 });
    v.insert("#anon_preprocessor-rule-disabled-statements_2".to_string(), ContextId { index: 3867 });
    v.insert("data-structures-definition-common-end".to_string(), ContextId { index: 3901 });
    v.insert("expressions".to_string(), ContextId { index: 3913 });
    v.insert("block".to_string(), ContextId { index: 3892 });
    v.insert("early-expressions".to_string(), ContextId { index: 3912 });
    v.insert("#anon_preprocessor-other_1".to_string(), ContextId { index: 3853 });
    v.insert("function-definition-body".to_string(), ContextId { index: 3915 });
    v.insert("preprocessor-block-if-branch-statements".to_string(), ContextId { index: 3942 });
    v.insert("preprocessor-if-branch-global".to_string(), ContextId { index: 3951 });
    v.insert("#anon_unique-types_2".to_string(), ContextId { index: 3888 });
    v.insert("data-structures-union-definition".to_string(), ContextId { index: 3909 });
    v.insert("#anon_objc-structures_1".to_string(), ContextId { index: 3845 });
    v.insert("data-structures-union-definition-block-start".to_string(), ContextId { index: 3911 });
    v
  }
} }

#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "C".to_string(),
  file_extensions: vec!["c".to_string(),"h".to_string()],
  scope: Scope { a: 844480764706816, b: 0 },
  first_line_match: Some("-[*]-( Mode:)? C -[*]-".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("hex_suffix".to_string(), "[g-zG-Z_][[:alnum:]_]*".to_string());
    v.insert("integer_suffix".to_string(), "[lL]{1,2}[uU]?|[uU][lL]{0,2}".to_string());
    v.insert("microsoft_types".to_string(), "__int8|__int16|__int32|__int64".to_string());
    v.insert("before_tag".to_string(), "struct|union|enum".to_string());
    v.insert("stdint".to_string(), "int8_t|int16_t|int32_t|int64_t|uint8_t|uint16_t|uint32_t|uint64_t|int_least8_t|int_least16_t|int_least32_t|int_least64_t|uint_least8_t|uint_least16_t|uint_least32_t|uint_least64_t|int_fast8_t|int_fast16_t|int_fast32_t|int_fast64_t|uint_fast8_t|uint_fast16_t|uint_fast32_t|uint_fast64_t|intptr_t|uintptr_t|intmax_t|intmax_t|uintmax_t|uintmax_t".to_string());
    v.insert("hex_exponent".to_string(), "(?:[pP][-+]??\\d+)".to_string());
    v.insert("control_keywords".to_string(), "break|case|continue|default|do|else|for|goto|if|_Pragma|return|switch|while".to_string());
    v.insert("compiler_directive".to_string(), "inline|restrict|__restrict__|__restrict".to_string());
    v.insert("declspec".to_string(), "__declspec\\(\\s*\\w+(?:\\([^)]+\\))?\\s*\\)".to_string());
    v.insert("identifier".to_string(), "\\b[[:alpha:]_][[:alnum:]_]*\\b".to_string());
    v.insert("macro_identifier".to_string(), "\\b[[:upper:]_][[:upper:][:digit:]_]{2,}\\b".to_string());
    v.insert("modifiers".to_string(), "{{storage_classes}}|{{type_qualifier}}|{{compiler_directive}}".to_string());
    v.insert("storage_classes".to_string(), "static|extern|register|{{declspec}}".to_string());
    v.insert("windows_types".to_string(), "APIENTRY|ATOM|BOOL|BOOLEAN|BYTE|CALLBACK|CCHAR|CHAR|COLORREF|CONST|DWORD|DWORDLONG|DWORD_PTR|DWORD32|DWORD64|FLOAT|HACCEL|HALF_PTR|HANDLE|HBITMAP|HBRUSH|HCOLORSPACE|HCONV|HCONVLIST|HCURSOR|HDC|HDDEDATA|HDESK|HDROP|HDWP|HENHMETAFILE|HFILE|HFONT|HGDIOBJ|HGLOBAL|HHOOK|HICON|HINSTANCE|HKEY|HKL|HLOCAL|HMENU|HMETAFILE|HMODULE|HMONITOR|HPALETTE|HPEN|HRESULT|HRGN|HRSRC|HSZ|HWINSTA|HWND|INT|INT_PTR|INT8|INT16|INT32|INT64|LANGID|LCID|LCTYPE|LGRPID|LONG|LONGLONG|LONG_PTR|LONG32|LONG64|LPARAM|LPBOOL|LPBYTE|LPCOLORREF|LPCSTR|LPCTSTR|LPCVOID|LPCWSTR|LPDWORD|LPHANDLE|LPINT|LPLONG|LPSTR|LPTSTR|LPVOID|LPWORD|LPWSTR|LRESULT|PBOOL|PBOOLEAN|PBYTE|PCHAR|PCSTR|PCTSTR|PCWSTR|PDWORD|PDWORDLONG|PDWORD_PTR|PDWORD32|PDWORD64|PFLOAT|PHALF_PTR|PHANDLE|PHKEY|PINT|PINT_PTR|PINT8|PINT16|PINT32|PINT64|PLCID|PLONG|PLONGLONG|PLONG_PTR|PLONG32|PLONG64|POINTER_32|POINTER_64|POINTER_SIGNED|POINTER_UNSIGNED|PSHORT|PSIZE_T|PSSIZE_T|PSTR|PTBYTE|PTCHAR|PTSTR|PUCHAR|PUHALF_PTR|PUINT|PUINT_PTR|PUINT8|PUINT16|PUINT32|PUINT64|PULONG|PULONGLONG|PULONG_PTR|PULONG32|PULONG64|PUSHORT|PVOID|PWCHAR|PWORD|PWSTR|QWORD|SC_HANDLE|SC_LOCK|SERVICE_STATUS_HANDLE|SHORT|SIZE_T|SSIZE_T|TBYTE|TCHAR|UCHAR|UHALF_PTR|UINT|UINT_PTR|UINT8|UINT16|UINT32|UINT64|ULONG|ULONGLONG|ULONG_PTR|ULONG32|ULONG64|UNICODE_STRING|USHORT|USN|VOID|WCHAR|WINAPI|WORD|WPARAM".to_string());
    v.insert("dec_exponent".to_string(), "(?:[eE][-+]??\\d+)".to_string());
    v.insert("non_func_keywords".to_string(), "if|for|switch|while|decltype|sizeof|__declspec|__attribute__".to_string());
    v.insert("basic_types".to_string(), "asm|__asm__|auto|bool|_Bool|char|_Complex|double|float|_Imaginary|int|long|short|signed|unsigned|void".to_string());
    v.insert("dec_suffix".to_string(), "[a-zA-Z_][[:alnum:]_]*".to_string());
    v.insert("type_qualifier".to_string(), "const|volatile".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("preprocessor-rule-enabled-data-structures".to_string(), ContextId { index: 815 });
    v.insert("data-structures-union-definition-block-start".to_string(), ContextId { index: 759 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 699 });
    v.insert("string_escaped_char".to_string(), ContextId { index: 822 });
    v.insert("preprocessor-block-if-branch-global".to_string(), ContextId { index: 789 });
    v.insert("types".to_string(), ContextId { index: 826 });
    v.insert("preprocessor-global".to_string(), ContextId { index: 799 });
    v.insert("#anon_preprocessor-comments_0".to_string(), ContextId { index: 700 });
    v.insert("#anon_numbers_1".to_string(), ContextId { index: 696 });
    v.insert("access-illegal".to_string(), ContextId { index: 739 });
    v.insert("keywords-parens".to_string(), ContextId { index: 774 });
    v.insert("#anon_preprocessor-rule-disabled-statements_1".to_string(), ContextId { index: 720 });
    v.insert("#anon_preprocessor-rule-enabled-statements_0".to_string(), ContextId { index: 728 });
    v.insert("#anon_preprocessor-other_0".to_string(), ContextId { index: 708 });
    v.insert("preprocessor-line-ending".to_string(), ContextId { index: 806 });
    v.insert("preprocessor-rule-other-global".to_string(), ContextId { index: 818 });
    v.insert("data-structures".to_string(), ContextId { index: 746 });
    v.insert("string_placeholder".to_string(), ContextId { index: 823 });
    v.insert("#anon_function-definition-params_0".to_string(), ContextId { index: 686 });
    v.insert("comments".to_string(), ContextId { index: 744 });
    v.insert("main".to_string(), ContextId { index: 777 });
    v.insert("#anon_typedef_0".to_string(), ContextId { index: 735 });
    v.insert("#anon_preprocessor-comments_1".to_string(), ContextId { index: 701 });
    v.insert("data-structures-struct-definition-after-name".to_string(), ContextId { index: 755 });
    v.insert("preprocessor-block-finish-if-branch-global".to_string(), ContextId { index: 786 });
    v.insert("#anon_preprocessor-rule-disabled-global_0".to_string(), ContextId { index: 716 });
    v.insert("negated-block".to_string(), ContextId { index: 780 });
    v.insert("#anon_block_0".to_string(), ContextId { index: 675 });
    v.insert("data-structures-definition-common-macro".to_string(), ContextId { index: 750 });
    v.insert("#anon_data-structures-struct-definition-block-start_0".to_string(), ContextId { index: 681 });
    v.insert("modifiers".to_string(), ContextId { index: 778 });
    v.insert("#anon_data-structures-union-definition-block-start_0".to_string(), ContextId { index: 682 });
    v.insert("#anon_preprocessor-other_4".to_string(), ContextId { index: 712 });
    v.insert("#anon_case-default_0".to_string(), ContextId { index: 677 });
    v.insert("preprocessor-comments".to_string(), ContextId { index: 791 });
    v.insert("#anon_preprocessor-rule-disabled-global_2".to_string(), ContextId { index: 718 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 733 });
    v.insert("#anon_function-definition-params_1".to_string(), ContextId { index: 687 });
    v.insert("data-structures-union-definition-after-name".to_string(), ContextId { index: 758 });
    v.insert("early-expressions".to_string(), ContextId { index: 761 });
    v.insert("__main".to_string(), ContextId { index: 736 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_1".to_string(), ContextId { index: 714 });
    v.insert("__start".to_string(), ContextId { index: 737 });
    v.insert("keywords".to_string(), ContextId { index: 773 });
    v.insert("c99".to_string(), ContextId { index: 742 });
    v.insert("typedef".to_string(), ContextId { index: 825 });
    v.insert("modifiers-parens".to_string(), ContextId { index: 779 });
    v.insert("preprocessor-rule-enabled-global".to_string(), ContextId { index: 816 });
    v.insert("global-maybe-function".to_string(), ContextId { index: 768 });
    v.insert("operators".to_string(), ContextId { index: 782 });
    v.insert("preprocessor-practical-workarounds".to_string(), ContextId { index: 811 });
    v.insert("data-structures-struct-definition-block-start".to_string(), ContextId { index: 756 });
    v.insert("block".to_string(), ContextId { index: 740 });
    v.insert("data-structures-struct-definition".to_string(), ContextId { index: 754 });
    v.insert("preprocessor-convention-ignore-uppercase-ident-lines".to_string(), ContextId { index: 793 });
    v.insert("data-structures-enum-definition".to_string(), ContextId { index: 751 });
    v.insert("preprocessor-expressions".to_string(), ContextId { index: 798 });
    v.insert("preprocessor-other".to_string(), ContextId { index: 810 });
    v.insert("#anon_numbers_2".to_string(), ContextId { index: 697 });
    v.insert("preprocessor-if-branch-function-call-arguments-finish".to_string(), ContextId { index: 802 });
    v.insert("#anon_preprocessor-rule-enabled-global_0".to_string(), ContextId { index: 725 });
    v.insert("preprocessor-rule-enabled-statements".to_string(), ContextId { index: 817 });
    v.insert("#anon_modifiers-parens_0".to_string(), ContextId { index: 691 });
    v.insert("preprocessor-elif-else-branch-global".to_string(), ContextId { index: 796 });
    v.insert("#anon_numbers_3".to_string(), ContextId { index: 698 });
    v.insert("function-definition-continue".to_string(), ContextId { index: 765 });
    v.insert("function-definition-body".to_string(), ContextId { index: 764 });
    v.insert("preprocessor-elif-else-branch-statements".to_string(), ContextId { index: 797 });
    v.insert("#anon_preprocessor-rule-enabled-global_2".to_string(), ContextId { index: 727 });
    v.insert("data-structures-body".to_string(), ContextId { index: 747 });
    v.insert("preprocessor-block-finish-statements".to_string(), ContextId { index: 788 });
    v.insert("global-modifier".to_string(), ContextId { index: 769 });
    v.insert("access".to_string(), ContextId { index: 738 });
    v.insert("#anon_preprocessor-rule-enabled-statements_1".to_string(), ContextId { index: 729 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_0".to_string(), ContextId { index: 722 });
    v.insert("variables".to_string(), ContextId { index: 827 });
    v.insert("preprocessor-rule-disabled-global".to_string(), ContextId { index: 813 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 679 });
    v.insert("#anon_numbers_0".to_string(), ContextId { index: 695 });
    v.insert("preprocessor-line-continuation".to_string(), ContextId { index: 805 });
    v.insert("statements".to_string(), ContextId { index: 821 });
    v.insert("#anon_preprocessor-disabled_0".to_string(), ContextId { index: 704 });
    v.insert("#anon_function-call_0".to_string(), ContextId { index: 683 });
    v.insert("#anon_strings_1".to_string(), ContextId { index: 734 });
    v.insert("function-call".to_string(), ContextId { index: 763 });
    v.insert("#anon_function-definition-body_0".to_string(), ContextId { index: 685 });
    v.insert("#anon_global-type_0".to_string(), ContextId { index: 688 });
    v.insert("#anon_function-call_1".to_string(), ContextId { index: 684 });
    v.insert("#anon_modifiers-parens_2".to_string(), ContextId { index: 693 });
    v.insert("#anon_preprocessor-convention-ignore-uppercase-calls-without-semicolon_0".to_string(), ContextId { index: 702 });
    v.insert("#anon_preprocessor-rule-disabled-statements_2".to_string(), ContextId { index: 721 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_1".to_string(), ContextId { index: 723 });
    v.insert("label".to_string(), ContextId { index: 775 });
    v.insert("late-expressions".to_string(), ContextId { index: 776 });
    v.insert("parens".to_string(), ContextId { index: 783 });
    v.insert("preprocessor-convention-ignore-uppercase-calls-without-semicolon".to_string(), ContextId { index: 792 });
    v.insert("preprocessor-macro-define".to_string(), ContextId { index: 807 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 678 });
    v.insert("#anon_preprocessor-rule-other-global_0".to_string(), ContextId { index: 731 });
    v.insert("#anon_preprocessor-other_1".to_string(), ContextId { index: 709 });
    v.insert("preprocessor-statements".to_string(), ContextId { index: 820 });
    v.insert("case-default".to_string(), ContextId { index: 743 });
    v.insert("preprocessor-if-branch-function-call".to_string(), ContextId { index: 800 });
    v.insert("strings".to_string(), ContextId { index: 824 });
    v.insert("incomplete-inc".to_string(), ContextId { index: 772 });
    v.insert("#anon_preprocessor-other_2".to_string(), ContextId { index: 710 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_0".to_string(), ContextId { index: 713 });
    v.insert("#anon_preprocessor-rule-enabled-data-structures_2".to_string(), ContextId { index: 724 });
    v.insert("data-structures-enum-definition-block-start".to_string(), ContextId { index: 753 });
    v.insert("preprocessor-block-finish-if-branch-statements".to_string(), ContextId { index: 787 });
    v.insert("brackets".to_string(), ContextId { index: 741 });
    v.insert("decimal-suffix".to_string(), ContextId { index: 760 });
    v.insert("#anon_preprocessor-macro-define_1".to_string(), ContextId { index: 706 });
    v.insert("#anon_preprocessor-other_3".to_string(), ContextId { index: 711 });
    v.insert("preprocessor-if-branch-function-call-arguments".to_string(), ContextId { index: 801 });
    v.insert("preprocessor-rule-other-statements".to_string(), ContextId { index: 819 });
    v.insert("#anon_keywords-parens_0".to_string(), ContextId { index: 690 });
    v.insert("preprocessor-block-finish-global".to_string(), ContextId { index: 785 });
    v.insert("preprocessor-if-branch-statements".to_string(), ContextId { index: 804 });
    v.insert("#anon_preprocessor-rule-enabled-statements_2".to_string(), ContextId { index: 730 });
    v.insert("preprocessor-data-structures".to_string(), ContextId { index: 794 });
    v.insert("preprocessor-rule-disabled-statements".to_string(), ContextId { index: 814 });
    v.insert("#anon_global-type_1".to_string(), ContextId { index: 689 });
    v.insert("preprocessor-block-if-branch-statements".to_string(), ContextId { index: 790 });
    v.insert("#anon_preprocessor-macro-define_0".to_string(), ContextId { index: 705 });
    v.insert("#anon_preprocessor-rule-enabled-global_1".to_string(), ContextId { index: 726 });
    v.insert("#anon_preprocessor-rule-disabled-data-structures_2".to_string(), ContextId { index: 715 });
    v.insert("#anon_preprocessor-macro-params_0".to_string(), ContextId { index: 707 });
    v.insert("preprocessor-macro-definition".to_string(), ContextId { index: 808 });
    v.insert("pragma-mark".to_string(), ContextId { index: 784 });
    v.insert("#anon_data-structures-enum-definition-block-start_0".to_string(), ContextId { index: 680 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 676 });
    v.insert("constants".to_string(), ContextId { index: 745 });
    v.insert("hexadecimal-suffix".to_string(), ContextId { index: 771 });
    v.insert("data-structures-union-definition".to_string(), ContextId { index: 757 });
    v.insert("expressions".to_string(), ContextId { index: 762 });
    v.insert("#anon_preprocessor-rule-other-statements_0".to_string(), ContextId { index: 732 });
    v.insert("function-definition-params".to_string(), ContextId { index: 766 });
    v.insert("numbers".to_string(), ContextId { index: 781 });
    v.insert("#anon_negated-block_0".to_string(), ContextId { index: 694 });
    v.insert("global-type".to_string(), ContextId { index: 770 });
    v.insert("#anon_preprocessor-rule-disabled-global_1".to_string(), ContextId { index: 717 });
    v.insert("#anon_preprocessor-rule-disabled-statements_0".to_string(), ContextId { index: 719 });
    v.insert("global".to_string(), ContextId { index: 767 });
    v.insert("data-structures-definition-common-begin".to_string(), ContextId { index: 748 });
    v.insert("preprocessor-if-branch-global".to_string(), ContextId { index: 803 });
    v.insert("preprocessor-rule-disabled-data-structures".to_string(), ContextId { index: 812 });
    v.insert("data-structures-definition-common-end".to_string(), ContextId { index: 749 });
    v.insert("preprocessor-macro-params".to_string(), ContextId { index: 809 });
    v.insert("#anon_modifiers-parens_1".to_string(), ContextId { index: 692 });
    v.insert("#anon_preprocessor-convention-ignore-uppercase-ident-lines_0".to_string(), ContextId { index: 703 });
    v.insert("data-structures-enum-definition-after-name".to_string(), ContextId { index: 752 });
    v.insert("preprocessor-disabled".to_string(), ContextId { index: 795 });
    v
  }
} }
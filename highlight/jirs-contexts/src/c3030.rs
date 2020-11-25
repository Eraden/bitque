
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
      regex: Regex::new("\\b(?i:(defun|defmethod|defmacro))\\b\\s+([\\w\\-!?<>]*)"),
      scope: vec![
        Scope {
            a: 46444131369484288,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576522690606,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615086,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:zerop|yes-or-no-p|y-or-n-p|write-to-string|write-string|write-char|write-byte|write|with-standard-io-syntax|with-slots|with-simple-restart|with-package-iterator|with-output-to-string|with-open-stream|with-open-file|with-input-from-string|with-hash-table-iterator|with-condition-restarts|with-compilation-unit|with-accessors|wild-pathname-p|warn|vectorp|vector-push-extend|vector-push|vector-pop|vector|variable-information|values-list|values|user-homedir-pathname|use-value|use-package|upper-case-p|upgraded-complex-part-type|upgraded-array-element-type|update-instance-for-redefined-class|update-instance-for-different-class|unuse-package|untrace|until-if|until|unread-char|union|unintern|unexport|typep|type-of|type-error-expected-type|type-error-datum|two-way-stream-output-stream|two-way-stream-input-stream|truncate|truename|tree-equal|translate-pathname|translate-logical-pathname|trace|to-alter|time|third|terpri|terminate-producing|tenth|tanh|tan|tailp|synonym-stream-symbol|symbolp|symbol-value|symbol-plist|symbol-package|symbol-name|symbol-function|sxhash|svref|summing|sum|subtypep|substitute-if-not|substitute-if|substitute|subst-if-not|subst-if|subst|subsetp|subseries|subseq|sublis|stringp|string>=|string>|string=|string<=|string<|string/=|string-upcase|string-trim|string-right-trim|string-not-lessp|string-not-greaterp|string-not-equal|string-lessp|string-left-trim|string-greaterp|string-equal|string-downcase|string-char-p|string-capitalize|string|streamp|stream-external-format|stream-error-stream|stream-element-type|store-value|step|standard-char-p|stable-sort|sqrt|split-if|split|special-form-p|sort|some|software-version|software-type|slot-value|slot-unbound|slot-missing|slot-makunbound|slot-exists-p|slot-boundp|sleep|sixth|sinh|sin|simple-vector-p|simple-string-p|simple-condition-format-string|simple-condition-format-arguments|simple-bit-vector-p|signum|signal|short-site-name|shiftf|shared-initialize|shadowing-import|shadow|seventh|setq|setf|set-syntax-from-char|set-pprint-dispatch|set-macro-character|set-exclusive-or|set-dispatch-macro-character|set-difference|set-char-bit|set|series|second|search|schar|scan-symbols|scan-sublists|scan-range|scan-plist|scan-multiple|scan-lists-of-lists-fringe|scan-lists-of-lists|scan-hash|scan-fn-inclusive|scan-fn|scan-file|scan-alist|scan|scale-float|sbit|rplacd|rplaca|row-major-aref|round|rotatef|room|reverse|revappend|result-of|restart-name|restart-case|restart-bind|rest|require|replace|rename-package|rename-file|remprop|remove-method|remove-duplicates|remove|remhash|remf|reinitialize-instance|reduce|realpart|realp|readtablep|readtable-case|read-preserving-whitespace|read-line|read-from-string|read-delimited-list|read-char-no-hang|read-char|read-byte|read|rationalp|rationalize|rational|rassoc-if-not|rassoc-if|rassoc|random-state-p|random|quote|pushnew|push|psetq|psetf|provide|progn|prog2|prog1|producing|proclaim|probe-file|print-unreadable-object|print-object|print|prin1|previous|pprint-tabular|pprint-tab|pprint-pop|pprint-newline|pprint-logical-block|pprint-linear|pprint-indent|pprint-fill|pprint-exit-if-list-exhausted|pprint-dispatch|positions|position-if-not|position-if|position|pop|plusp|phase|peek-char|pathnamep|pathname-version|pathname-type|pathname-name|pathname-match-p|pathname-host|pathname-directory|pathname-device|pathname|parse-namestring|parse-macro|parse-integer|pairlis|packagep|package-used-by-list|package-use-list|package-shadowing-symbols|package-nicknames|package-name|package-error-package|output-stream-p|open-stream-p|open|oddp|nunion|numerator|numberp|nthcdr|nth-value|nth|nsubstitute-if-not|nsubstitute-if|nsubstitute|nsubst-if-not|nsubst-if|nsubst|nsublis|nstring-upcase|nstring-downcase|nstring-capitalize|nset-exclusive-or|nset-difference|nreverse|nreconc|notevery|notany|no-next-method|no-applicable-method|ninth|nintersection|next-out|next-method-p|next-in|nconcing|nconc|nbutlast|namestring|name-char|multiple-value-setq|multiple-value-list|multiple-value-bind|muffle-warning|mismatch|minusp|minimizing|minimize|mingle|method-qualifiers|method-combination-error|merge-pathnames|merge|memberp|member-if-not|member-if|member|maximizing|maximize|mask-field|mask|mapping|maphash|map-into|map-fn|map|makunbound|make-two-way-stream|make-synonym-stream|make-symbol|make-string-output-stream|make-string-input-stream|make-string|make-sequence|make-random-state|make-pathname|make-package|make-load-form-saving-slots|make-load-form|make-list|make-instances-obsolete|make-instance|make-hash-table|make-echo-stream|make-dispatch-macro-character|make-condition|make-concatenated-stream|make-char|make-broadcast-stream|make-array|macroexpand-1|macroexpand|macro-function|machine-version|machine-type|machine-instance|lower-case-p|loop-finish|long-site-name|logtest|logorc2|logorc1|lognot|lognand|logical-pathname-translations|logical-pathname|logcount|logbitp|logandc2|logandc1|log|locally|load-logical-pathname-translations|load|listp|listen|list-length|list-all-packages|list*|list|lisp-implementation-version|lisp-implementation-type|length|ldiff|ldb-test|ldb|lcm|latch|last|lambda|keywordp|iterate|isqrt|invoke-restart|invoke-debugger|invalid-method-error|intersection|intern|interactive-stream-p|integerp|integer-length|integer-decode-float|int-char|inspect|input-stream-p|initialize-instance|in-package|import|imagpart|ignore-errors|identity|host-namestring|hash-table-test|hash-table-size|hash-table-rehash-threshold|hash-table-rehash-size|hash-table-p|hash-table-count|handler-case|handler-bind|graphic-char-p|gethash|getf|get-universal-time|get-setf-method-multiple-value|get-setf-method|get-properties|get-output-stream-string|get-internal-run-time|get-internal-real-time|get-decoded-time|get|gentemp|gensym|generic-function|generator|gcd|gathering|gatherer|functionp|function-lambda-expression|function-keywords|function-information|funcall|fourth|formatter|format|floor|floatp|float-sign|float-radix|float-precision|float-digits|float|first|finish-output|find-symbol|find-restart|find-package|find-method|find-if-not|find-if|find-class|find-all-symbols|find|fill-pointer|fill|file-write-date|file-string-length|file-position|file-namestring|file-length|file-error-pathname|file-author|fifth|ffloor|fdefinition|fboundp|f|expt|export|expand|exp|every|evenp|evalhook|eval|error|ensure-generic-function|enough-namestring|endp|encode-universal-time|enclose|encapsulated|elt|eighth|ed|echo-stream-output-stream|echo-stream-input-stream|ecase|dribble|dpb|documentation|do-symbols|do-external-symbols|do-all-symbols|disassemble|directory-namestring|directory|digit-char-p|digit-char|destructuring-bind|describe-object|describe|deposit-field|denominator|delete-package|delete-if-not|delete-if|delete-file|delete-duplicates|delete|defvar|deftype|defstruct|defpackage|define-setf-method|define-modify-macro|define-method-combination|define-declaration|define-condition|define-compiler-macro|defgeneric|defclass|decode-universal-time|decode-float|declaration-information|declaim|counting|count-if-not|count-if|count|cotruncate|cosh|cos|copy-tree|copy-symbol|copy-seq|copy-readtable|copy-pprint-dispatch|copy-list|copy-alist|continue|constantp|consp|cons|conjugate|concatenated-stream-streams|concatenate|compute-restarts|compute-applicable-methods|complexp|complex|complement|compiler-macroexpand-1|compiler-macroexpand|compiler-macro-function|compiler-let|compiled-function-p|compile-file-pathname|compile-file|compile|commonp|collecting-fn|collecting|collect-sum|collect-plist|collect-or|collect-nth|collect-nconc|collect-min|collect-max|collect-length|collect-last|collect-hash|collect-fn|collect-first|collect-file|collect-append|collect-and|collect-alist|collect|coerce|code-char|clrhash|close|clear-input|class-of|class-name|cis|chunk|choose-if|choose|check-type|characterp|character|char>=|char>|char=|char<=|char<|char/=|char-upcase|char-not-lessp|char-not-greaterp|char-not-equal|char-name|char-lessp|char-int|char-greaterp|char-font|char-equal|char-downcase|char-code|char-bits|char-bit|char|change-class|cerror|cell-error-name|ceiling|cdr|cddr|cdddr|cddddr|cdddar|cddar|cddadr|cddaar|cdar|cdadr|cdaddr|cdadar|cdaar|cdaadr|cdaaar|ccase|catenate|car|call-next-method|call-method|cadr|caddr|cadddr|caddar|cadar|cadadr|cadaar|caar|caadr|caaddr|caadar|caaar|caaadr|caaaar|byte-size|byte-position|byte|butlast|broadcast-stream-streams|break|boundp|both-case-p|boole|bit-xor|bit-vector-p|bit-orc2|bit-orc1|bit-not|bit-nor|bit-nand|bit-ior|bit-eqv|bit-andc2|bit-andc1|bit-and|bit|augment-environment|atom|atanh|atan|assoc-if-not|assoc-if|assoc|assert|asinh|asin|ash|arrayp|array-total-size|array-row-major-index|array-rank|array-in-bounds-p|array-has-fill-pointer-p|array-element-type|array-dimensions|array-dimension|arithmetic-error-operation|arithmetic-error-operands|aref|apropos-list|apropos|applyhook|apply|appending|append|alter|alphanumericp|alpha-char-p|adjustable-array-p|adjust-array|adjoin|add-method|acosh|acos|acons|abs|abort)\\b"),
      scope: vec![
        Scope {
            a: 61925255088570368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
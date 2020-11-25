
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "PureScript".to_string(),
  file_extensions: vec!["purs".to_string()],
  scope: Scope { a: 845004750716928, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("random-data".to_string(), ContextId { index: 8787 });
    v.insert("#anon_strings_0".to_string(), ContextId { index: 8732 });
    v.insert("regex".to_string(), ContextId { index: 8789 });
    v.insert("prelude-function-category".to_string(), ContextId { index: 8779 });
    v.insert("prelude-function-read".to_string(), ContextId { index: 8783 });
    v.insert("trace".to_string(), ContextId { index: 8802 });
    v.insert("global-function".to_string(), ContextId { index: 8756 });
    v.insert("monoid-typeclass".to_string(), ContextId { index: 8769 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 8729 });
    v.insert("eff".to_string(), ContextId { index: 8742 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 8723 });
    v.insert("regex-data".to_string(), ContextId { index: 8790 });
    v.insert("__main".to_string(), ContextId { index: 8733 });
    v.insert("maybe-data".to_string(), ContextId { index: 8762 });
    v.insert("prelude-data".to_string(), ContextId { index: 8773 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 8727 });
    v.insert("maybe-function".to_string(), ContextId { index: 8763 });
    v.insert("module-name".to_string(), ContextId { index: 8764 });
    v.insert("monoid-function".to_string(), ContextId { index: 8768 });
    v.insert("prelude".to_string(), ContextId { index: 8772 });
    v.insert("prelude-function-alternative".to_string(), ContextId { index: 8775 });
    v.insert("prelude-function-boollike".to_string(), ContextId { index: 8778 });
    v.insert("reserveds".to_string(), ContextId { index: 8792 });
    v.insert("__start".to_string(), ContextId { index: 8734 });
    v.insert("array-constant".to_string(), ContextId { index: 8736 });
    v.insert("strings".to_string(), ContextId { index: 8800 });
    v.insert("trace-data".to_string(), ContextId { index: 8803 });
    v.insert("trace-function".to_string(), ContextId { index: 8804 });
    v.insert("generic-tyvar".to_string(), ContextId { index: 8753 });
    v.insert("tuple-function".to_string(), ContextId { index: 8806 });
    v.insert("prelude-function-num".to_string(), ContextId { index: 8781 });
    v.insert("decl-ctor".to_string(), ContextId { index: 8741 });
    v.insert("regex-function".to_string(), ContextId { index: 8791 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 8728 });
    v.insert("reserveds-other".to_string(), ContextId { index: 8794 });
    v.insert("enum-typeclass".to_string(), ContextId { index: 8749 });
    v.insert("data".to_string(), ContextId { index: 8740 });
    v.insert("string-function".to_string(), ContextId { index: 8799 });
    v.insert("either-function".to_string(), ContextId { index: 8746 });
    v.insert("maybe".to_string(), ContextId { index: 8761 });
    v.insert("prelude-function-show".to_string(), ContextId { index: 8785 });
    v.insert("type-simple".to_string(), ContextId { index: 8809 });
    v.insert("array".to_string(), ContextId { index: 8735 });
    v.insert("enum-function".to_string(), ContextId { index: 8748 });
    v.insert("operator".to_string(), ContextId { index: 8771 });
    v.insert("error".to_string(), ContextId { index: 8750 });
    v.insert("prelude-function-applicative".to_string(), ContextId { index: 8776 });
    v.insert("string".to_string(), ContextId { index: 8798 });
    v.insert("monoid".to_string(), ContextId { index: 8767 });
    v.insert("eff-data".to_string(), ContextId { index: 8743 });
    v.insert("array-op".to_string(), ContextId { index: 8737 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 8726 });
    v.insert("main".to_string(), ContextId { index: 8760 });
    v.insert("st-function".to_string(), ContextId { index: 8797 });
    v.insert("type-constraint".to_string(), ContextId { index: 8807 });
    v.insert("prelude-function".to_string(), ContextId { index: 8774 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 8731 });
    v.insert("ioref-function".to_string(), ContextId { index: 8759 });
    v.insert("random".to_string(), ContextId { index: 8786 });
    v.insert("reserveds-control".to_string(), ContextId { index: 8793 });
    v.insert("global-constant".to_string(), ContextId { index: 8755 });
    v.insert("either-data".to_string(), ContextId { index: 8745 });
    v.insert("error-function".to_string(), ContextId { index: 8751 });
    v.insert("global".to_string(), ContextId { index: 8754 });
    v.insert("monad".to_string(), ContextId { index: 8765 });
    v.insert("prelude-function-bits".to_string(), ContextId { index: 8777 });
    v.insert("st-data".to_string(), ContextId { index: 8796 });
    v.insert("st".to_string(), ContextId { index: 8795 });
    v.insert("strings-escapes".to_string(), ContextId { index: 8801 });
    v.insert("ioref-data".to_string(), ContextId { index: 8758 });
    v.insert("numbers".to_string(), ContextId { index: 8770 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 8730 });
    v.insert("random-function".to_string(), ContextId { index: 8788 });
    v.insert("comments".to_string(), ContextId { index: 8738 });
    v.insert("ffi".to_string(), ContextId { index: 8752 });
    v.insert("type-signature".to_string(), ContextId { index: 8808 });
    v.insert("#anon_ffi_0".to_string(), ContextId { index: 8724 });
    v.insert("tuple".to_string(), ContextId { index: 8805 });
    v.insert("prelude-function-ref".to_string(), ContextId { index: 8784 });
    v.insert("prelude-function-monad".to_string(), ContextId { index: 8780 });
    v.insert("ioref".to_string(), ContextId { index: 8757 });
    v.insert("either".to_string(), ContextId { index: 8744 });
    v.insert("enum".to_string(), ContextId { index: 8747 });
    v.insert("monad-function".to_string(), ContextId { index: 8766 });
    v.insert("constants".to_string(), ContextId { index: 8739 });
    v.insert("prelude-function-ord".to_string(), ContextId { index: 8782 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 8725 });
    v
  }
} }
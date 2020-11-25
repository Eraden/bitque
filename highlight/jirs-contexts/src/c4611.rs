
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
      regex: Regex::new("(?x)\\b__(?:\n  # unary operators\n  invert|neg|pos|abs|\n  # binary operators\n  add|and|div|divmod|floordiv|lshift|mod|mul|or|pow|rshift|sub|truediv|xor|\n  contains|\n  # right-hand binary operators\n  radd|rand|rdiv|rdivmod|rfloordiv|rlshift|rmod|rmul|ror|rpow|rrshift|rsub|rtruediv|rxor|\n  # in-place operator assignments\n  iadd|iand|idiv|ifloordiv|ilshift|imod|imul|ior|ipow|irshift|isub|itruediv|ixor|\n  # comparisons\n  eq|ge|gt|le|lt|ne|\n  cmp|rcmp| # py2\n  # primary coercion\n  bool|str|\n  nonzero|unicode| # py2\n  # number coercion (converts something to a number)\n  bytes|complex|float|index|int|round|\n  long| # py2\n  # other \"coercion\"\n  format|len|length_hint|hash|repr|reversed|\n  coerce|hex|oct| # py2\n  fspath|\n  # iterator (and \'await\')\n  iter|next|\n  aiter|anext|\n  await|\n  # attribute and item access\n  delattr|delitem|delslice|\n  getattr|getattribute|getitem|getslice|\n  setattr|setitem|setslice|\n  dir|missing|\n  # context manager\n  enter|exit|\n  aenter|aexit|\n  # other class magic\n  call|del|init|new|init_subclass|\n  instancecheck|subclasscheck|\n  # pickling\n  getnewargs|getnewargs_ex|getstate|setstate|reduce|reduce_ex|\n  # descriptors\n  delete|get|set|set_name|\n  # class-specific\n  subclasses|\n  # dataclasses (PEP 557)\n  post_init|\n  # for typing core support (PEP 560)\n  class_getitem|mro_entries\n)__\\b"),
      scope: vec![
        Scope {
            a: 61925255101481022,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }
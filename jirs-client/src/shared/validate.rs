#[macro_export]
macro_rules! validator {
    ($id: ident, $is_valid: expr, $msg: expr) => {
        #[derive(Debug, Default)]
        pub struct $id;

        impl Validator for $id {
            fn validate(&mut self, value: &str) -> bool {
                $is_valid(value)
            }

            fn message(&self) -> &'static str {
                if self.is_valid() {
                    ""
                } else {
                    $msg
                }
            }
        }
    };
    ($id: ident, $const_name: ident, $const_ty: ty, $const_cond: ident, $msg: expr) => {
        #[derive(Debug, Default)]
        pub struct $id<const $const_name: $const_ty>;

        impl<const $const_name: $const_ty> Validator for $id<$const_name> {
            fn validate(&mut self, value: &str) -> bool {
                $const_cond::<$const_name>(value)
            }

            fn message(&self) -> &'static str {
                if self.is_valid() {
                    ""
                } else {
                    $msg
                }
            }
        }
    };
}

pub trait Validator: std::fmt::Debug {
    fn validate(&mut self, value: &str) -> bool;

    fn is_valid(&self) -> bool {
        false
    }

    fn message(&self) -> &'static str {
        ""
    }
}

#[derive(Debug, Default)]
pub struct Chain<A, B>(A, B)
where
    A: Validator,
    B: Validator;

impl<A, B> Validator for Chain<A, B>
where
    A: Validator,
    B: Validator,
{
    fn validate(&mut self, value: &str) -> bool {
        // trigger both
        let left = self.0.validate(value);
        let right = self.1.validate(value);
        left && right
    }

    fn is_valid(&self) -> bool {
        self.0.is_valid() && self.1.is_valid()
    }

    fn message(&self) -> &'static str {
        match (!self.0.is_valid(), !self.1.is_valid()) {
            (true, _) => self.0.message(),
            (_, true) => self.1.message(),
            _ => "",
        }
    }
}

#[derive(Debug, Default)]
pub struct AlwaysValid;

impl Validator for AlwaysValid {
    fn validate(&mut self, _: &str) -> bool {
        true
    }

    fn is_valid(&self) -> bool {
        true
    }
}

fn is_more_eq<const MIN: usize>(value: &str) -> bool {
    value.len() >= MIN
}

fn is_less_eq<const MAX: usize>(value: &str) -> bool {
    value.len() <= MAX
}

validator!(AtLeast, MIN, usize, is_more_eq, "Value is too short");
validator!(AtMost, MAX, usize, is_less_eq, "Value is too long");

pub type Between<const MIN: usize, const MAX: usize> =
    Chain<Changed<AtLeast<MIN>>, Changed<AtMost<MAX>>>;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum TouchState {
    Untouched,
    Touched,
}

impl Default for TouchState {
    fn default() -> Self {
        TouchState::Untouched
    }
}

#[derive(Debug, Default)]
pub struct Touched<Valid>(pub Valid, TouchState)
where
    Valid: Validator;

impl<V> Validator for Touched<V>
where
    V: Validator,
{
    fn validate(&mut self, value: &str) -> bool {
        self.1 = TouchState::Touched;
        self.0.validate(value)
    }

    fn is_valid(&self) -> bool {
        match self.1 {
            TouchState::Untouched => true,
            _ => self.0.is_valid(),
        }
    }

    fn message(&self) -> &'static str {
        self.0.message()
    }
}

#[derive(Debug)]
pub enum ChangedState {
    Value(bool),
    Unset,
}

impl Default for ChangedState {
    fn default() -> Self {
        ChangedState::Unset
    }
}

#[derive(Debug, Default)]
pub struct Changed<V>(V, ChangedState)
where
    V: Validator;

impl<V> Validator for Changed<V>
where
    V: Validator,
{
    fn validate(&mut self, value: &str) -> bool {
        let res = self.0.validate(value);
        self.1 = ChangedState::Value(res);
        res
    }

    fn is_valid(&self) -> bool {
        match self.1 {
            ChangedState::Value(b) => b,
            _ => self.0.is_valid(),
        }
    }

    fn message(&self) -> &'static str {
        self.0.message()
    }
}

pub trait Validate<V: Validator> {
    fn validator_mut(&mut self) -> Option<&mut V>;

    fn validator(&self) -> Option<&V>;

    fn is_valid(&self) -> bool {
        self.validator().map(|v| v.is_valid()).unwrap_or(true)
    }

    fn validate(&mut self, value: &str) -> bool {
        self.validator_mut()
            .map(|v| v.validate(value))
            .unwrap_or(true)
    }

    fn err_msg(&self) -> &'static str {
        self.validator().map(|v| v.message()).unwrap_or("")
    }
}

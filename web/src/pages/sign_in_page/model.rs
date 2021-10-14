use crate::shared::validate::*;
use crate::validations::*;
use crate::validator;

validator!(EmailFormat, is_email, "Not a valid e-mail address");
validator!(UuidFormat, is_token, "Malformed token");

pub type UsernameValidator = Touched<Between<4, 36>>;
pub type EmailValidator = Touched<Chain<Changed<AtLeast<6>>, Changed<EmailFormat>>>;
pub type TokenValidator = Touched<Chain<Between<10, 36>, Changed<UuidFormat>>>;

#[derive(Debug)]
pub enum SignInMsg {
    AuthenticateSuccess,
    InvalidPair,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum SignInState {
    Initial,
    RequestSend,
    EmailSend,
    InvalidPair,
}

impl Default for SignInState {
    fn default() -> Self {
        Self::Initial
    }
}

#[derive(Debug, Default)]
pub struct SignInPage {
    pub username: String,
    pub email: String,
    pub token: String,
    pub bad_token: String,
    // validators
    pub username_v: UsernameValidator,
    pub email_v: EmailValidator,
    pub token_v: TokenValidator,

    pub state: SignInState,
}

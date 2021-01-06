#[derive(Debug, Default)]
pub struct SignInPage {
    pub username: String,
    pub email: String,
    pub token: String,
    pub login_success: bool,
    pub bad_token: String,
    // touched
    pub username_touched: bool,
    pub email_touched: bool,
    pub token_touched: bool,
}

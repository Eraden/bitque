#[derive(Debug, Default)]
pub struct SignUpPage {
    pub username: String,
    pub email: String,
    pub sign_up_success: bool,
    pub error: String,
    // touched
    pub username_touched: bool,
    pub email_touched: bool,
}

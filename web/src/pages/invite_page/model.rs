#[derive(Debug, Default)]
pub struct InvitePage {
    pub token: String,
    pub token_touched: bool,
    pub error: Option<String>,
}

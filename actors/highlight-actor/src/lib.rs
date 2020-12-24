use {
    actix::{Actor, Handler, SyncContext},
    std::sync::Arc,
    syntect::{
        easy::HighlightLines,
        highlighting::{Style, ThemeSet},
        parsing::SyntaxSet,
    },
};

mod load;

lazy_static::lazy_static! {
    pub static ref THEME_SET: Arc<ThemeSet> = Arc::new(load::integrated_themeset());
    pub static ref SYNTAX_SET: Arc<SyntaxSet> = Arc::new(load::integrated_syntaxset());
}

#[derive(Debug)]
pub enum HighlightError {
    UnknownLanguage,
    UnknownTheme,
    ResultUnserializable,
}

fn hi<'l>(code: &'l str, lang: &'l str) -> Result<Vec<(Style, &'l str)>, HighlightError> {
    let set = SYNTAX_SET
        .as_ref()
        .find_syntax_by_name(lang)
        .ok_or_else(|| HighlightError::UnknownLanguage)?;
    let theme: &syntect::highlighting::Theme = THEME_SET
        .as_ref()
        .themes
        .get("GitHub")
        .ok_or_else(|| HighlightError::UnknownTheme)?;

    let mut hi = HighlightLines::new(set, theme);
    Ok(hi.highlight(code, SYNTAX_SET.as_ref()))
}

#[derive(Debug, Default)]
pub struct HighlightActor {}

impl Actor for HighlightActor {
    type Context = SyncContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "Result<Vec<u8>, HighlightError>")]
pub struct HighlightCode {
    pub code: String,
    pub lang: String,
}

impl Handler<HighlightCode> for HighlightActor {
    type Result = Result<Vec<u8>, HighlightError>;

    fn handle(&mut self, msg: HighlightCode, _ctx: &mut Self::Context) -> Self::Result {
        let res = hi(&msg.code, &msg.lang)?;
        bincode::serialize(&res).map_err(|_| HighlightError::ResultUnserializable)
    }
}

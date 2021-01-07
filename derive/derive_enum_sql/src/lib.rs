extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

fn to_lower_case(s: &str) -> String {
    let mut lower = String::new();
    for (idx, c) in s.chars().enumerate() {
        if idx > 0 && c.is_uppercase() {
            lower.push('_');
        }
        lower.push_str(c.to_lowercase().to_string().as_str());
    }
    lower
}

fn into_str(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
#[cfg(feature = "backend")]
impl diesel::serialize::ToSql<{name}Type, diesel::pg::Pg> for {name} {{
    fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, diesel::pg::Pg>) -> diesel::serialize::Result {{
        match *self {{
"#,
        name = name,
    );

    for variant in variants {
        let lower = to_lower_case(&variant);
        code.push_str(
            format!(
                "            {name}::{variant} => out.write_all(b\"{lower}\")?,\n",
                variant = variant,
                name = name,
                lower = lower
            )
            .as_str(),
        );
    }
    code.push_str("        };\n        Ok(diesel::serialize::IsNull::No)\n    }\n}");
    code
}

fn from_str(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
#[cfg(feature = "backend")]
impl {name} {{
    fn from_diesel_bytes(bytes: Option<&[u8]>) -> diesel::deserialize::Result<{name}> {{
        match diesel::not_none!(bytes) {{
"#,
        name = name,
    );

    for variant in variants {
        let lower = to_lower_case(&variant);
        code.push_str(
            format!(
                "            b\"{lower}\" => Ok({name}::{variant}),\n",
                variant = variant,
                name = name,
                lower = lower
            )
            .as_str(),
        );
    }

    code.push_str(format!("            _ => Ok({name}::default()),", name = name).as_str());
    code.push_str("        }\n    }\n}");
    code.push_str(
        format!(
            r#"
#[cfg(feature = "backend")]
impl diesel::deserialize::FromSql<{name}Type, diesel::pg::Pg> for {name} {{
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {{
        {name}::from_diesel_bytes(bytes)
    }}
}}

#[cfg(feature = "backend")]
impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg> for {name} {{
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {{
        {name}::from_diesel_bytes(bytes)
    }}
}}
    "#,
            name = name
        )
        .as_str(),
    );
    code
}

#[proc_macro_derive(EnumSql)]
pub fn derive_enum_sql(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter().peekable();
    while let Some(token) = it.peek() {
        if let TokenTree::Ident(_) = token {
            break;
        } else {
            it.next();
        }
    }
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "pub" {
            panic!("Expect to find keyword pub but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword pub but nothing was found")
    }
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "enum" {
            panic!("Expect to find keyword struct but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword struct but nothing was found")
    }
    let name = it
        .next()
        .expect("Expect to struct name but nothing was found")
        .to_string();

    let mut variants = vec![];
    if let Some(TokenTree::Group(group)) = it.next() {
        for token in group.stream() {
            if let TokenTree::Ident(ident) = token {
                variants.push(ident.to_string())
            }
        }
    } else {
        panic!("Enum variants group expected");
    }
    if variants.is_empty() {
        panic!("Enum cannot be empty")
    }

    let mut code = format!(
        r#"
#[cfg(feature = "backend")]
#[derive(diesel::SqlType)]
#[postgres(type_name = "{name}Type")]
pub struct {name}Type;

#[cfg(feature = "backend")]
impl diesel::query_builder::QueryId for {name}Type {{
    type QueryId = {name};
}}
    "#,
        name = name
    );

    code.push_str(from_str(&name, &variants).as_str());
    code.push_str(into_str(&name, &variants).as_str());

    code.parse().unwrap()
}

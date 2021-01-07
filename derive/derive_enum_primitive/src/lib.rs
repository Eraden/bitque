extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

fn into_str(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
#[cfg(feature = "frontend")]
impl {name} {{
    pub fn to_str(&self) -> &'static str {{
        match self {{
"#,
        name = name,
    );

    for variant in variants {
        let lower = variant.to_lowercase();
        code.push_str(
            format!(
                "            {name}::{variant} => \"{lower}\",\n",
                variant = variant,
                name = name,
                lower = lower
            )
            .as_str(),
        );
    }
    code.push_str("        }\n    }\n}");
    code
}

fn from_str(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
#[cfg(feature = "frontend")]
impl FromStr for {name} {{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
"#,
        name = name,
    );

    for variant in variants {
        let lower = variant.to_lowercase();
        code.push_str(
            format!(
                "            \"{lower}\" => Ok({name}::{variant}),\n",
                variant = variant,
                name = name,
                lower = lower
            )
            .as_str(),
        );
    }

    code.push_str(
        format!(
            "            _ => Err(format!(\"Unknown {name} {{}}\", s)),",
            name = name
        )
        .as_str(),
    );
    code.push_str("        }\n    }\n}");
    code
}

fn into_label(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
#[cfg(feature = "frontend")]
impl {name} {{
    pub fn to_label(&self) -> &'static str {{
        match self {{
"#,
        name = name,
    );

    for variant in variants {
        code.push_str(
            format!(
                "            {name}::{variant} => \"{variant}\",\n",
                variant = variant,
                name = name,
            )
            .as_str(),
        );
    }
    code.push_str("        }\n    }\n}");
    code
}

fn into_u32(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
impl Into<u32> for {name} {{
    fn into(self) -> u32 {{
        match self {{
"#,
        name = name
    );
    for (idx, variant) in variants.iter().enumerate() {
        code.push_str(
            format!(
                "            {name}::{variant} => {idx},\n",
                variant = variant,
                name = name,
                idx = idx
            )
            .as_str(),
        );
    }
    code.push_str("        }\n    }\n}");
    code
}

fn from_u32(name: &str, variants: &[String]) -> String {
    let mut code = format!(
        r#"
impl Into<{name}> for u32 {{
    fn into(self) -> {name} {{
        match self {{
"#,
        name = name
    );
    for (idx, variant) in variants.iter().enumerate() {
        code.push_str(
            format!(
                "      {idx} => {name}::{variant},\n",
                variant = variant,
                name = name,
                idx = idx
            )
            .as_str(),
        );
    }
    code.push_str(format!("      _ => {name}::default(),\n", name = name,).as_str());
    code.push_str("    }\n  }\n}");
    code
}

#[proc_macro_derive(EnumPrimitive)]
pub fn derive_enum_primitive(item: TokenStream) -> TokenStream {
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

    let mut code = String::new();

    code.push_str(from_str(&name, &variants).as_str());
    code.push_str(into_str(&name, &variants).as_str());
    code.push_str(into_label(&name, &variants).as_str());
    code.push_str(into_u32(&name, &variants).as_str());
    code.push_str(from_u32(&name, &variants).as_str());

    code.parse().unwrap()
}

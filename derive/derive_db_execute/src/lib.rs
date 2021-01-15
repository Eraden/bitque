mod parse_attr;
mod utils;

extern crate proc_macro;

use crate::parse_attr::Attributes;
use proc_macro::{token_stream::IntoIter, TokenStream, TokenTree};
use std::iter::Peekable;

fn parse_meta(mut it: Peekable<IntoIter>) -> (Peekable<IntoIter>, Option<Attributes>) {
    let mut attrs: Option<Attributes> = None;
    while let Some(token) = it.peek() {
        match token {
            // lookup for attr
            TokenTree::Punct(p) if p.as_char() == '#' => {
                let res = parse_attr::parse(it);
                it = res.0;
                attrs = res.1;
            }
            TokenTree::Ident(_) => {
                break;
            }
            _ => {
                eprintln!("skip token {:#?}", token);
                it.next();
            }
        };
    }
    (it, attrs)
}

#[proc_macro_derive(Execute, attributes(db_exec))]
pub fn derive_enum_iter(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter().peekable();
    let res = parse_meta(it);
    it = res.0;
    let attrs = res.1.expect("Result meta attribute is required");
    let result = attrs
        .result
        .expect("Meta attribute `result` is required. Try add db_exec(result = \"foo\")");
    let schema = attrs
        .schema
        .expect("Meta attribute `schema` is required. Try add db_exec(schema = \"foo\")");

    it = utils::skip_pub(it);
    it = utils::skip_struct(it);

    let name = it
        .next()
        .expect("Expect to struct name but nothing was found")
        .to_string();

    let action_result = if attrs.load.is_some() {
        format!("Vec<{}>", result)
    } else if attrs.destroy.is_some() {
        "usize".to_string()
    } else {
        result.clone()
    };

    let query = if let Some(q) = attrs.find {
        build_find_exec(&name, &result, &schema, &q, &action_result)
    } else if let Some(q) = attrs.load {
        build_load_exec(&name, &result, &schema, &q, &action_result)
    } else if let Some(q) = attrs.update {
        build_update_exec(&name, &result, &schema, &q, &action_result)
    } else if let Some(q) = attrs.destroy {
        build_destroy_exec(&name, &result, &schema, &q, &action_result)
    } else if let Some(q) = attrs.create {
        build_create_exec(&name, &result, &schema, &q, &action_result)
    } else {
        "".to_string()
    };

    let code = format!(
        r#"
        impl actix::Message for {name} {{
            type Result = Result<{action_result}, crate::DatabaseError>;
        }}

        impl actix::Handler<{name}> for crate::DbExecutor {{
            type Result = Result<{action_result}, crate::DatabaseError>;

            fn handle(&mut self, msg: {name}, _ctx: &mut Self::Context) -> Self::Result {{
                let conn = crate::db_pool!(self);

                msg.execute(conn)
            }}
        }}
        
        {query}
    "#,
        name = name,
        query = query,
        action_result = action_result
    );
    code.parse().unwrap()
}

fn build_create_exec(
    name: &str,
    resource: &str,
    schema: &str,
    query: &str,
    action_result: &str,
) -> String {
    format!(
        r#"
    impl {name} {{
            pub fn execute(
                self,
                conn: &crate::DbPooledConn,
            ) -> Result<{action_result}, crate::DatabaseError> {{
                crate::Guard::new(conn)?.run(|_guard| {{
                    use crate::schema::{schema}::dsl::*;
                    let msg = self;
                    crate::q!({query}).get_result(conn).map_err(|e| {{
                        log::error!("{{:?}}", e);
                        crate::DatabaseError::GenericFailure(
                            crate::OperationError::Create,
                            crate::ResourceKind::{resource},
                        )
                    }})
                }})
            }}
        }}
    "#,
        name = name,
        schema = schema,
        query = query,
        resource = resource,
        action_result = action_result
    )
}

fn build_find_exec(
    name: &str,
    resource: &str,
    schema: &str,
    query: &str,
    action_result: &str,
) -> String {
    format!(
        r#"
    impl {name} {{
            pub fn execute(
                self,
                conn: &crate::DbPooledConn,
            ) -> Result<{action_result}, crate::DatabaseError> {{
                use crate::schema::{schema}::dsl::*;
                let msg = self;
                crate::q!({query}).first(conn).map_err(|e| {{
                    log::error!("{{:?}}", e);
                    crate::DatabaseError::GenericFailure(
                        crate::OperationError::LoadSingle,
                        crate::ResourceKind::{resource},
                    )
                }})
            }}
        }}
    "#,
        name = name,
        schema = schema,
        query = query,
        resource = resource,
        action_result = action_result
    )
}

fn build_load_exec(
    name: &str,
    resource: &str,
    schema: &str,
    query: &str,
    action_result: &str,
) -> String {
    format!(
        r#"
    impl {name} {{
            pub fn execute(
                self,
                conn: &crate::DbPooledConn,
            ) -> Result<{action_result}, crate::DatabaseError> {{
                use crate::schema::{schema}::dsl::*;
                let msg = self;
                crate::q!({query}).load(conn).map_err(|e| {{
                    log::error!("{{:?}}", e);
                    crate::DatabaseError::GenericFailure(
                        crate::OperationError::LoadCollection,
                        crate::ResourceKind::{resource},
                    )
                }})
            }}
        }}
    "#,
        name = name,
        schema = schema,
        query = query,
        resource = resource,
        action_result = action_result
    )
}

fn build_update_exec(
    name: &str,
    resource: &str,
    schema: &str,
    query: &str,
    action_result: &str,
) -> String {
    format!(
        r#"
    impl {name} {{
            pub fn execute(
                self,
                conn: &crate::DbPooledConn,
            ) -> Result<{action_result}, crate::DatabaseError> {{
                use crate::schema::{schema}::dsl::*;
                let msg = self;
                crate::q!({query}).get_result(conn).map_err(|e| {{
                    log::error!("{{:?}}", e);
                    crate::DatabaseError::GenericFailure(
                        crate::OperationError::Update,
                        crate::ResourceKind::{resource},
                    )
                }})
            }}
        }}
    "#,
        name = name,
        schema = schema,
        query = query,
        resource = resource,
        action_result = action_result
    )
}

fn build_destroy_exec(
    name: &str,
    resource: &str,
    schema: &str,
    query: &str,
    action_result: &str,
) -> String {
    format!(
        r#"
    impl {name} {{
            pub fn execute(
                self,
                conn: &crate::DbPooledConn,
            ) -> Result<{action_result}, crate::DatabaseError> {{
                use crate::schema::{schema}::dsl::*;
                let msg = self;
                crate::q!({query}).execute(conn).map_err(|e| {{
                    log::error!("{{:?}}", e);
                    crate::DatabaseError::GenericFailure(
                        crate::OperationError::Delete,
                        crate::ResourceKind::{resource},
                    )
                }})
            }}
        }}
    "#,
        name = name,
        schema = schema,
        query = query,
        resource = resource,
        action_result = action_result
    )
}

#[macro_use]
extern crate reexport_proc_macro;

pub use {actix_cors, actix_rt, actix_service, actix_web, actix_web_actors, serde};

reexport_proc_macro!(actix_derive);
reexport_proc_macro!(serde_derive);

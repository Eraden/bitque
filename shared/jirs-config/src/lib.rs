#[cfg(feature = "aws-s3")]
pub mod amazon;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "local-storage")]
pub mod fs;

#[cfg(feature = "hi")]
pub mod hi;

#[cfg(feature = "mail")]
pub mod mail;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "websocket")]
pub mod websocket;

pub mod utils;

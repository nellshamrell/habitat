extern crate hyper;
#[macro_use] extern crate log;
extern crate tempdir;
extern crate mustache;
extern crate rustc_serialize;
extern crate toml;
extern crate ansi_term;
extern crate regex;
extern crate libc;

pub mod error;
pub mod command;
pub mod util;
pub mod pkg;
pub mod discovery;
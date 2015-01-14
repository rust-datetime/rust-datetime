#![crate_name = "datetime"]
#![crate_type = "dylib"]
#![feature(plugin)]
#![allow(unstable)]

extern crate regex;

mod now;
mod parse;
pub mod local;
pub mod instant;
pub mod duration;
pub mod format;

pub mod cli;
pub mod operations;
pub mod utils;
include!(concat!(env!("OUT_DIR"), "/domain.rs"));

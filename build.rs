use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not found");
    let dest_path = Path::new(&out_dir).join("domain.rs");
    let mut f = File::create(&dest_path).expect("Unable to create file");

    let input = File::open(".env").expect(".env file not found");
    let buffered = BufReader::new(input);

    let mut domain: Option<String> = None;
    let mut proxy: Option<String> = None;

    for line in buffered.lines() {
        let line = line.unwrap();
        if line.starts_with("DOMAIN=") {
            domain = Some(line[7..].to_string());
        }
        if line.starts_with("PROXY=") {
            proxy = Some(line[6..].to_string());
        }
    }

    if let Some(d) = domain {
        writeln!(f, "pub const DOMAIN: &'static str = \"{}\";", d).unwrap();
    }
    if let Some(p) = proxy {
        writeln!(f, "pub const PROXY: Option<&'static str> = Some(\"{}\");", p).unwrap();
    }
}

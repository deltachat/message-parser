use std::io::{self, Read, Write};

use parser::parse_markdown_text;
#[allow(dead_code)]
mod parser;
extern crate nom;
#[macro_use]
extern crate serde_derive;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    //println!("input: {:?}", buffer);

    let output = parse_markdown_text(&buffer);

    io::stdout().write_all(format!("output: {:?}", output).as_bytes())?;

    //println!("output: {:?}", output);
    Ok(())
}

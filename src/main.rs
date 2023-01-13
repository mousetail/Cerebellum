mod ast;
mod parser;
use crate::ast::operator_macro::Operator;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser)]
struct Args {
    filename: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.filename)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let ast = parser::to_ast(
        file_contents
            .split_ascii_whitespace()
            .map(|i| ast::AstSymbol::from_verbose_name(i))
            .collect::<Vec<_>>()
            .as_slice(),
        &mut 0,
    );

    println!("{:?}", ast);

    Ok(())
}

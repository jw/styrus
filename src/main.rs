extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate log;
extern crate pretty_env_logger;

use clap::Clap;
use std::fs;

mod parser;
use crate::parser::parse;

mod compiler;
use crate::compiler::compile;

#[derive(Debug, Clone)]
pub enum AstNode {
    Asterisk(bool),
    Prefix(String),
    Identifier(String),
    Separator(String),
    Selector(Vec<AstNode>),

    Property {
        words: Vec<String>,
    },

    Rule {
        selectors: Vec<AstNode>,
        properties: Vec<AstNode>,
    },
}

#[derive(Clap)]
#[clap(version = "0.1.1", author = "Jan Willems <jw@elevenbits.com>")]
struct Opts {
    source: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    pretty_env_logger::init();
    let opts: Opts = Opts::parse();
    if opts.verbose > 0 {
        println!("Compiling {}...", opts.source);
    }
    log::info!("Compiling {}...", opts.source);
    let unparsed_file = fs::read_to_string(opts.source).expect("cannot read stylus file");
    let ast = parse(&unparsed_file).expect("unsuccessful parse");
    log::info!("AST: {:#?}", &ast);
    let css = compile(ast);
    println!("{}", css);
}

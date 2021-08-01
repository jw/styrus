extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::error::Error;
use pest::Parser;


#[derive(Parser)]
#[grammar = "stylus.pest"]
struct StylusParser;


#[derive(Debug)]
pub enum AstNode {
    Selector(String),
}


fn main() {
    let unparsed_file = fs::read_to_string("stylus.stylus")
        .expect("cannot read stylus file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:?}", &astnode);
}


fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>>{
    let mut ast = vec![];

    let pairs = StylusParser::parse(Rule::stylus, &source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::rule => {
                ast.push(AstNode::Selector(pair.as_str().to_string()));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(ast)
}

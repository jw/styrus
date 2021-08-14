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
    Property {
        name: String,
        value: String,
    },
    Selector {
        ident: String,
        properties: Vec<AstNode>,
    },
}

fn main() {
    let unparsed_file = fs::read_to_string("stylus.stylus").expect("cannot read stylus file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:?}", &astnode);
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let rules = StylusParser::parse(Rule::stylus, source)?.next().unwrap();

    for rule in rules.into_inner() {
        match rule.as_rule() {
            Rule::rule => {
                let mut inner_rules = rule.into_inner();
                let selector = AstNode::Selector {
                    ident: inner_rules.next().unwrap().as_str().to_string(),
                    properties: build_ast_from_property_lines(inner_rules),
                };
                ast.push(selector);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    Ok(ast)
}

fn build_ast_from_property_lines(pairs: pest::iterators::Pairs<Rule>) -> Vec<AstNode> {
    let mut ast = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::propertyLine => {
                for property in pair.into_inner() {
                    ast.push(build_ast_for_property(property));
                }
            }
            _ => unreachable!(),
        }
    }
    ast
}

fn build_ast_for_property(property: pest::iterators::Pair<Rule>) -> AstNode {
    let mut inner_rules = property.into_inner();
    AstNode::Property {
        name: inner_rules.next().unwrap().as_str().to_string(),
        value: inner_rules.next().unwrap().as_str().to_string(),
    }
}

#[cfg(test)]
mod tests {

    use pest::consumes_to;
    use pest::parses_to;

    #[derive(Parser)]
    #[grammar = "stylus.pest"]
    struct StylusParser;

    #[test]
    fn simple() {
        parses_to! {
        parser: StylusParser,
            input: "abc:\n  def = klm",
            rule: Rule::stylus,
            tokens: [
                stylus(0, 16, [
                    rule(0, 16, [
                        selector(0, 4, [
                                ident(0, 3),
                            ]
                        ),
                        propertyLine(7, 16, [
                            property(7, 16, [
                                name(7, 10),
                                value(13, 16),
                            ])
                        ])
                    ]),
                    EOI(16, 16),
                ])
            ]
        };
    }
}

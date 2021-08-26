extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate log;
extern crate pretty_env_logger;

use std::fs;

use clap::Clap;

use crate::AstNode::{Asterisk, Identifier, Prefix, Separator};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "styrus.pest"]
struct StylusParser;

#[derive(Debug)]
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
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("Result: {:?}", &astnode);
    log::info!("Result: {:#?}", &astnode)
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];
    let rules = StylusParser::parse(Rule::rules, source)?.next().unwrap();
    for rule in rules.into_inner() {
        match rule.as_rule() {
            Rule::rule => {
                ast.push(create_rule(rule));
            }
            _ => unreachable!(),
        }
    }
    Ok(ast)
}

fn create_rule(rule: Pair<Rule>) -> AstNode {
    let mut selectors = vec![];
    let mut properties = vec![];
    let inner_rules = rule.into_inner();
    for rule in inner_rules {
        match rule.as_rule() {
            Rule::selectors => {
                selectors = create_selectors(rule.into_inner());
            }
            Rule::properties => {
                properties = create_properties(rule);
            }
            _ => unreachable!(),
        }
    }
    AstNode::Rule {
        selectors,
        properties,
    }
}

fn create_selectors(rules: pest::iterators::Pairs<Rule>) -> Vec<AstNode> {
    let mut selectors = vec![];
    for rule in rules {
        match rule.as_rule() {
            Rule::selector => {
                let selector = create_selector(rule.into_inner());
                selectors.push(selector);
            }
            _ => unreachable!(),
        }
    }
    selectors
}

fn create_selector(rules: pest::iterators::Pairs<Rule>) -> AstNode {
    let mut nodes = vec![];
    for rule in rules {
        match rule.as_rule() {
            Rule::asterisk => {
                nodes.push(Asterisk(true));
            }
            Rule::prefix => {
                nodes.push(Prefix(rule.as_str().to_string()));
            }
            Rule::identifier => {
                nodes.push(Identifier(rule.as_str().to_string()));
            }
            Rule::separator => {
                nodes.push(Separator(rule.as_str().to_string()));
            }
            _ => unreachable!(),
        }
    }
    AstNode::Selector(nodes)
}

fn create_properties(rule: Pair<Rule>) -> Vec<AstNode> {
    let mut properties = vec![];
    for pair in rule.into_inner() {
        properties.push(create_property(pair))
    }
    properties
}

fn create_property(pair: Pair<Rule>) -> AstNode {
    AstNode::Property {
        words: vec![pair.as_str().to_string()],
    }
}

#[cfg(test)]
mod tests {

    use pest::consumes_to;
    use pest::parses_to;

    #[derive(Parser)]
    #[grammar = "styrus.pest"]
    struct StylusParser;

    #[test]
    fn nothing() {
        parses_to! {
            parser: StylusParser,
            input: "",
            rule: Rule::stylus,
            tokens: [
                stylus(0, 0, [
                    rules(0, 0, []),
                    EOI(0, 0)
                ])
            ]
        }
    }

    #[test]
    fn one_empty_line() {
        parses_to! {
            parser: StylusParser,
            input: "\n",
            rule: Rule::stylus,
            tokens: [
                stylus(0, 1, [
                    rules(0, 1, [
                        rule(0, 1, [])
                    ]),
                    EOI(1, 1)
                ])
            ]
        }
    }

    #[test]
    fn complete() {
        parses_to! {
            parser: StylusParser,
            input: "*h1 > p\n  border 1px\n\nh2\n  padding 1px 1px 1px 1px\n",
            rule: Rule::stylus,
            tokens: [
                stylus(0, 51, [
                    rules(0, 51, [
                        rule(0, 21, [
                            selectors(0, 7, [
                                selector(0, 1, [
                                    asterisk(0, 1)
                                ]),
                                selector(1, 5, [
                                    identifier(1, 3, []),
                                    separator(4, 5, [])
                                ]),
                                selector(6, 7, [
                                    identifier(6, 7, [])
                                ])
                            ]),
                            properties(7, 20, [
                                property(7, 20, [
                                    indent(7, 10, [])
                                ])
                            ])
                        ]),
                        rule(21, 22, []),
                        rule(22, 51, [
                            selectors(22, 24, [
                                selector(22, 24, [
                                    identifier(22, 24)
                                ])
                            ]),
                            properties(24, 50, [
                                property(24, 50, [
                                    indent(24, 27)
                                ])
                            ])
                        ])
                    ]),
                    EOI(51, 51),
                ])
            ]
        }
    }
}

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate log;
extern crate pretty_env_logger;

use std::fs;

use clap::Clap;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "styrus.pest"]
struct StylusParser;

#[derive(Debug)]
pub enum AstNode {
    Property {
        // name: String,
        // values: Vec<String>,
        words: Vec<String>,
    },
    Selector {
        words: Vec<String>,
    },
    Rule {
        selector: Box<AstNode>,
        properties: Vec<AstNode>,
    },
    NOP {},
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
    // println!("RULES: {:#?}", rules);
    for rule in rules.into_inner() {
        match rule.as_rule() {
            Rule::rule => {
                log::debug!("Creating rule...");
                ast.push(create_rule(rule));
            }
            _ => unreachable!(),
        }
    }
    Ok(ast)
}

fn create_rule(rule: Pair<Rule>) -> AstNode {
    log::info!("rule: {:?}", rule);
    let mut selector = AstNode::Selector { words: vec![] };
    let mut properties = vec![];
    let inner_rules = rule.into_inner();
    println!("RULES: {:?}", inner_rules.clone());
    log::info!("selector and properties: {:?}", inner_rules);
    for rule in inner_rules {
        match rule.as_rule() {
            Rule::selector => {
                selector = create_selector(rule);
            }
            Rule::properties => {
                properties = create_properties(rule);
            }
            _ => unreachable!(),
        }
    }
    AstNode::Rule {
        selector: Box::new(selector),
        properties,
    }
}

fn create_selector(rule: Pair<Rule>) -> AstNode {
    let selector = AstNode::Selector {
        words: vec![rule.as_str().to_string()],
    };
    selector
}

fn create_properties(rule: Pair<Rule>) -> Vec<AstNode> {
    let mut properties = vec![];
    for pair in rule.into_inner() {
        log::info!("-> pair: {:?}", pair);
        properties.push(create_property(pair))
    }
    properties
}

fn create_property(pair: Pair<Rule>) -> AstNode {
    let property = AstNode::Property {
        // name: name,
        // values: values
        words: vec![pair.as_str().to_string()],
    };
    property
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
    fn selector_property() {
        parses_to! {
            parser: StylusParser,
            input: "selector\n    property\n",
            rule: Rule::stylus,
            tokens: [
                stylus(0, 22, [
                    rules(0, 22, [
                        rule(0, 22, [
                            selector(0, 8),
                            properties(8, 21, [
                                property(8, 21, [
                                    indent(8, 13)
                                ])
                            ])
                        ])
                    ]),
                    EOI(22, 22)
                ])
            ]
        }
    }

    #[test]
    fn complete() {
        parses_to! {
            parser: StylusParser,
            input: "selector\n    property\n\n\nfoo bar\n  fizz fuzz\n",
            rule: Rule::stylus,
            tokens: [
                stylus(0, 44, [
                    rules(0, 44, [
                        rule(0, 22, [
                            selector(0, 8),
                            properties(8, 21, [
                                property(8, 21, [
                                    indent(8, 13)
                                ])
                            ])
                        ]),
                        rule(22, 23),
                        rule(23, 24),
                        rule(24, 44, [
                            selector(24, 31),
                            properties(31, 43, [
                                property(31, 43, [
                                    indent(31, 34)
                                ])
                            ])
                        ])
                    ]),
                    EOI(44, 44)
                ])
            ]
        }
    }
}

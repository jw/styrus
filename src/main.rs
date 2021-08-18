extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use clap::Clap;

use pest::error::Error;
use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "indent.pest"]
struct StylusParser;

#[derive(Debug)]
pub enum AstNode {
    Property {
        name: String,
        values: Vec<String>,
    },
    Selector {
        word: String,
    },
    Rule {
        selector: Box<AstNode>,
        properties: Vec<AstNode>,
    }
}

#[derive(Clap)]
#[clap(version = "0.1.1", author = "Jan Willems <jw@elevenbits.com>")]
struct Opts {
    source: String,
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    if opts.verbose > 0 {
        println!("Compiling {}...", opts.source);
    }
    let unparsed_file = fs::read_to_string(opts.source).expect("cannot read stylus file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("Result: {:#?}", &astnode);
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let rules = StylusParser::parse(Rule::stylus, source)?.next().unwrap();

    for rule in rules.into_inner() {
        println!("rule: {:?}", rule);
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
    println!("rule: {:?}", rule);
    let mut inner_rules = rule.into_inner();
    println!("Inner rules: {:?}", inner_rules);
    let rule = AstNode::Rule {
        selector: Box::from(AstNode::Selector { word: inner_rules.next().unwrap().as_str().to_string() }),
        properties: create_properties(inner_rules),
    };
    rule
}

fn create_properties(pairs: pest::iterators::Pairs<Rule>) -> Vec<AstNode> {
    let mut ast = vec![];
    for pair in pairs {
        println!("pair: {:?}", pair);
        match pair.as_rule() {
            Rule::property => {
                ast.push(create_property(pair.into_inner()));
            },
            Rule::indentation => {
                println!("Found indentation");
            }
            _ => unreachable!(),
        }
    }
    ast
}

fn create_property(pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let mut name = "".to_string();
    let mut values = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::name => {
                name = pair.as_str().to_string();
            },
            Rule::values => {
                values = create_values(pair.into_inner());
            },
            _ => unreachable!(),
        }
    }
    let property = AstNode::Property {
        name: name,
        values: values
    };
    property
}

fn create_values(pairs: pest::iterators::Pairs<Rule>) -> Vec<String> {
    let mut values = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::value => {
                values.push(pair.as_str().to_string());
            },
            _ => unreachable!(),
        }
    }
    values
}


    // let selector = AstNode::Rule {
    //     ident: inner_rules.next().unwrap().as_str().to_string(),
    //     properties: build_ast_from_property_lines(inner_rules),
    // };


// fn build_ast_from_property_lines(pairs: pest::iterators::Pairs<Rule>) -> Vec<AstNode> {
//     let mut ast = vec![];
//     for pair in pairs {
//         match pair.as_rule() {
//             Rule::propertyLine => {
//                 for property in pair.into_inner() {
//                     ast.push(build_ast_for_property(property));
//                 }
//             }
//             _ => unreachable!(),
//         }
//     }
//     ast
// }


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


    #[test]
    fn propertyLine() {
        parses_to! {
            parser: StylusParser,
            input: "one two",
            rule: Rule::propertyLine,
            tokens: [
                propertyLine(0, 7, [
                    property(0, 7, [
                        name(0, 7)
                    ])
                ])
            ]
        };
    }

}

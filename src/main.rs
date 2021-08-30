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

trait Visitor {
    fn visit(&mut self, node: &AstNode) -> String;
}

struct Compiler;
impl Visitor for Compiler {
    fn visit(&mut self, node: &AstNode) -> String {
        match node {
            AstNode::Asterisk(_) => "*".to_string(),
            AstNode::Prefix(prefix) => prefix.to_string(),
            AstNode::Separator(separator) => {
                format!(" {} ", separator.to_string())
            }
            AstNode::Identifier(identifier) => identifier.to_string(),
            AstNode::Selector(selectors) => {
                let mut out = String::new();
                for selector in selectors {
                    out = format!("{}{}", out, self.visit(selector));
                }
                out
            }
            AstNode::Property { words } => {
                let mut out = String::new();
                for word in words {
                    out = format!("{}{}", out, word);
                }
                out
            }
            AstNode::Rule {
                selectors,
                properties,
            } => {
                let mut out = String::new();
                if !selectors.is_empty() {
                    for selector in selectors {
                        out = format!("{}{}", out, self.visit(selector));
                    }
                    out += " {";
                    for word in properties {
                        out = format!("{}{}", out, self.visit(word));
                    }
                    out += "\n}"
                } else {
                    out = "\n".to_string();
                }
                out
            }
        }
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

fn compile(ast: Vec<AstNode>) -> String {
    let mut compiler = Compiler;
    let mut css = String::new();
    for node in ast {
        css = format!("{}{}", css, compiler.visit(&node));
    }
    css
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
                nodes.push(AstNode::Asterisk(true));
            }
            Rule::prefix => {
                nodes.push(AstNode::Prefix(rule.as_str().to_string()));
            }
            Rule::identifier => {
                nodes.push(AstNode::Identifier(rule.as_str().to_string()));
            }
            Rule::separator => {
                nodes.push(AstNode::Separator(rule.as_str().to_string()));
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

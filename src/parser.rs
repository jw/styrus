//! Parses the Stylus code into AstNodes.

use crate::AstNode;

#[derive(Parser)]
#[grammar = "styrus.pest"]
struct StylusParser;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];
    let rules = StylusParser::parse(Rule::rules, source)?.next().unwrap();
    log::debug!("RULES: {:#?}", rules);
    for rule in rules.into_inner() {
        log::debug!("RULE: {:?}", rule);
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
    let mut indent = 0usize;
    let mut name = "".to_string();
    let mut values = vec![];
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::indent => {
                indent = p.as_str().chars().count() - 1;
            }
            Rule::name => {
                name = p.as_str().to_string();
            }
            Rule::colon => {}
            Rule::value => {
                values.push(p.as_str().to_string());
            }
            _ => unreachable!(),
        }
    }
    AstNode::Property {
        indent,
        name,
        values,
    }
}

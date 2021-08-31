//! Compiles an AST of AstNodes to a css String.

use crate::AstNode;

trait Visitor {
    fn visit(&mut self, node: &AstNode) -> String;
}

pub struct Compiler;

impl Visitor for Compiler {
    fn visit(&mut self, node: &AstNode) -> String {
        match node {
            AstNode::Asterisk(_) => "*".to_string(),
            AstNode::Prefix(prefix) => prefix.to_string(),
            AstNode::Separator(separator) => {
                format!(" {} ", separator.to_string())
            }
            AstNode::Identifier(identifier) => identifier.to_string(),
            AstNode::Name(_) => "".to_string(),
            AstNode::Value(_) => "".to_string(),
            AstNode::Selector(selectors) => {
                let mut out = String::new();
                for selector in selectors {
                    out = format!("{}{}", out, self.visit(selector));
                }
                out
            }
            AstNode::Property {
                indent,
                name,
                values,
            } => {
                let mut out = format!("{:width$}", " ", width = *indent as usize);
                out += &name.to_string();
                out += ": ";
                for value in values {
                    out = format!("{}{} ", out, value);
                }
                out = out.strip_suffix(' ').unwrap().to_string();
                out += ";\n";
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
                    out += " {\n";
                    for word in properties {
                        out = format!("{}{}", out, self.visit(word));
                    }
                    out += "}"
                } else {
                    out = "\n".to_string();
                }
                out
            }
        }
    }
}

pub fn compile(ast: Vec<AstNode>) -> String {
    let mut compiler = Compiler;
    let mut css = String::new();
    for node in ast {
        css = format!("{}{}", css, compiler.visit(&node));
    }
    css
}

#[test]
fn nothing() {
    use crate::parser::parse;
    let ast = parse("").expect("unsuccessful parse");
    let css = compile(ast);
    assert_eq!(css, "");
}

#[test]
fn one_empty_line() {
    use crate::parser::parse;
    let ast = parse("\n").expect("unsuccessful parse");
    let css = compile(ast);
    assert_eq!(css, "\n");
}

#[test]
fn complete() {
    use crate::parser::parse;
    let ast = parse("*h1 > p\n  border 1px\n\nh2\n  padding 1px 1px 1px 1px\n")
        .expect("unsuccessful parse");
    let css = compile(ast);
    assert_eq!(
        css,
        "*h1 > p {\n  border: 1px;\n}\nh2 {\n  padding: 1px 1px 1px 1px;\n}"
    );
}

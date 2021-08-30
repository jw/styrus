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

pub fn compile(ast: Vec<AstNode>) -> String {
    let mut compiler = Compiler;
    let mut css = String::new();
    for node in ast {
        css = format!("{}{}", css, compiler.visit(&node));
    }
    css
}

use crate::{TokenReader, TokenType, ASTNode, ASTNodeType, simple_lexer::SimpleLexer};

pub struct SimpleCalcultor;

// 计算器，但计算结合性有问题
impl SimpleCalcultor {
    pub fn calculate(code: String) {
        let tree = Self::parse(code);
        Self::dump_ast_node(&tree, "".to_owned());
        Self::calc(&tree, "".to_owned());
    }

    fn parse(code: String) -> Box<dyn ASTNode> {
        let lexer = SimpleLexer::new();
        let tokens = lexer.tokenize(code);
        Self::init_tree(&mut Box::new(tokens))
    }

    fn calc(node: &Box<dyn ASTNode>, ident: String) -> i32 {
        let mut result: i32 = 0;
        println!("{}Caculating: {:?}", ident.clone(), node.get_type());
        match node.get_type() {
            ASTNodeType::Programm => {
                if let Some(child) = node.get_children().get(0) {
                    result = Self::calc(child, ident.clone() + "\t");
                }
            }
            ASTNodeType::Additive => {
                let child1 = node.get_children().get(0).unwrap();
                let value1 = Self::calc(child1, ident.clone() + "\t");
                let child2 = node.get_children().get(1).unwrap();
                let value2 = Self::calc(child2, ident.clone() + "\t");
                if node.get_text() == "+" {
                    result = value1 + value2;
                } else {
                    result = value1 - value2;
                }
            }
            ASTNodeType::Multiplicative => {
                let child1 = node.get_children().get(0).unwrap();
                let value1 = Self::calc(child1, ident.clone() + "\t");
                let child2 = node.get_children().get(1).unwrap();
                let value2 = Self::calc(child2, ident.clone() + "\t");
                if node.get_text() == "*" {
                    result = value1 * value2;
                } else {
                    result = value1 / value2;
                }
            }
            ASTNodeType::IntLiteral => {
                result = node.get_text().parse::<i32>().unwrap();
            }
            _ => {}
        }
        println!("{}Result: {:?}", ident.clone(), result);
        result
    }

    pub fn int_declare(mut tokens: &mut Box<dyn TokenReader>) -> Option<Box<dyn ASTNode>> {
        if let Some(token) = tokens.peek() {
            if token.get_type() == TokenType::Int {
                tokens.read();
                if let Some(token) = tokens.peek() {
                    if token.get_type() == TokenType::Identifier {
                        let mut node = SimpleASTNode::new(ASTNodeType::IntDeclaration, token.get_text());
                        tokens.read();
                        if let Some(token) = tokens.peek() {
                            if token.get_type() == TokenType::Assignment {
                                tokens.read();
                                if let Some(child) = Self::additive(&mut tokens) {
                                    node.add_children(child);
                                    return Some(Box::new(node));
                                }

                                if let Some(token) = tokens.peek() {
                                    if token.get_type() != TokenType::Assignment {
                                        return None;
                                    }
                                } else {
                                    return None;
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn init_tree(mut tokens: &mut Box<dyn TokenReader>) -> Box<dyn ASTNode> {
        let mut root = SimpleASTNode::new(ASTNodeType::Programm, "Calculator".to_owned());
        if let Some(child) = Self::additive(&mut tokens) {
            root.add_children(child);
        }
        Box::new(root)
    }

    fn additive(mut tokens: &mut Box<dyn TokenReader>) -> Option<Box<dyn ASTNode>> {
        let child1 = Self::multiplicative(tokens).unwrap();
        if let Some(token) = tokens.peek() {
            if token.get_type() == TokenType::Plus || token.get_type() == TokenType::Minus {
                tokens.read();
                let child2 = Self::additive(&mut tokens).unwrap();
                let mut node = SimpleASTNode::new(ASTNodeType::Additive, token.get_text());
                node.add_children(child1);
                node.add_children(child2);
                return Some(Box::new(node));
            }
        }
        Some(child1)
    }

    fn multiplicative(mut tokens: &mut Box<dyn TokenReader>) -> Option<Box<dyn ASTNode>> {
        let child1 = Self::primary(tokens).unwrap();
        if let Some(token) = tokens.peek() {
            if token.get_type() == TokenType::Star || token.get_type() == TokenType::Slash {
                tokens.read();
                let child2 = Self::multiplicative(&mut tokens).unwrap();
                let mut node = SimpleASTNode::new(ASTNodeType::Multiplicative, token.get_text());
                node.add_children(child1);
                node.add_children(child2);
                return Some(Box::new(node));
            }  
        }
        Some(child1)
    }

    fn primary(tokens: &mut Box<dyn TokenReader>) -> Option<Box<dyn ASTNode>> {
        if let Some(token) = tokens.peek() {
            if token.get_type() == TokenType::Identifier {
                tokens.read();
                return Some(Box::new(SimpleASTNode::new(ASTNodeType::Identifier, token.get_text())));
            } else if token.get_type() == TokenType::IntLiteral {
                tokens.read();
                return Some(Box::new(SimpleASTNode::new(ASTNodeType::IntLiteral, token.get_text())));
            } else if token.get_type() == TokenType::LeftParen {
                tokens.read();
                let node = Self::additive(tokens).unwrap();
                if let Some(token) = tokens.peek() {
                    if token.get_type() == TokenType::RightParen {
                        tokens.read();
                        return Some(node);
                    }
                }
            }
        }
        None
    }

    pub fn dump_ast_node(node: &Box<dyn ASTNode>, ident: String) {
        println!("{}{:?} {:?}", ident, node.get_type(), node.get_text());
        for child in node.get_children() {
            let ident = ident.clone() + "\t";
            Self::dump_ast_node(&child, ident);
        }
    }
}

pub struct SimpleASTNode {
    pub children: Vec<Box<dyn ASTNode>>,
    pub node_type: ASTNodeType,
    pub node_text: String,
}

impl ASTNode for SimpleASTNode {
    fn get_children(&self) -> &Vec<Box<dyn ASTNode>> {
        &self.children
    }

    fn get_type(&self) -> ASTNodeType {
        self.node_type.clone()
    }

    fn get_text(&self) -> String {
        self.node_text.clone()
    }
}

impl SimpleASTNode {
    pub fn new(node_type: ASTNodeType, node_text: String) -> Self {
        SimpleASTNode {
            children: Vec::new(),
            node_type,
            node_text,
        }
    }

    pub fn add_children(&mut self, child: Box<dyn ASTNode>) {
        self.children.push(child)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate() {
        let code = "int a = b + 3;".to_owned();
        let lexer = SimpleLexer::new();
        let mut tokens = lexer.tokenize(code);
        let node = SimpleCalcultor::int_declare(&mut tokens).unwrap();
        SimpleCalcultor::dump_ast_node(&node, "".to_owned());
        println!("------------------");

        let code = "2 + 3 * 5;".to_owned();
        SimpleCalcultor::calculate(code);
        println!("------------------");

        let code = "2 + 3 + 4;".to_owned();
        SimpleCalcultor::calculate(code);
        println!("------------------");
    }
}

use std::rc::Rc;
use anyhow::anyhow;
use crate::{TokenReader, TokenType, ASTNode, ASTNodeType, simple_lexer::SimpleLexer, ResultType};

pub struct SimpleParser;

// 语法解析器
impl SimpleParser {
    pub fn parse(code: String) -> Rc<dyn ASTNode> {
        let lexer = SimpleLexer::new();
        let tokens = lexer.tokenize(code);
        Self::program(&mut Box::new(tokens))
    }

    fn program(tokens: &mut Box<dyn TokenReader>) -> Rc<dyn ASTNode> {
        let mut node = SimpleASTNode::new(ASTNodeType::Programm, "pwc".to_owned());
        while let Some(_) = tokens.peek() {
            if let Ok(Some(child)) = Self::int_declare(tokens) {
                node.add_children(child);
            } else 
            if let Some(child) = Self::expression(tokens) {
                node.add_children(child);
            }
            else if let Ok(Some(child)) = Self::assignment(tokens) {
                node.add_children(child);
            } else {
                println!("not valid ast tree");
            }
        }
        Rc::new(node)
    }

    // 表达式，
    // 如 2 + 3;
    fn expression(tokens: &mut Box<dyn TokenReader>) -> Option<Rc<dyn ASTNode>> {
        let pos = tokens.get_position();
        if let Some(node) = Self::additive(tokens) {
            if let Some(token) = tokens.peek() {
                if token.get_type() == TokenType::SemiColon {
                    tokens.read();
                    return Some(node);
                } else {
                    tokens.set_position(pos);
                }
            } else {
                tokens.set_position(pos);
            }
        }
        None
    }

    // 赋值语句
    // 如 a = 1;
    // 如 a = 3 + 4;
    fn assignment(tokens: &mut Box<dyn TokenReader>) -> ResultType<Option<Rc<dyn ASTNode>>> {
        if let Some(token) = tokens.peek() {
            if token.get_type() == TokenType::Identifier {
                tokens.read();
                let mut node = SimpleASTNode::new(ASTNodeType::AssignmentStmt, token.get_text());
                if let Some(token) = tokens.peek() {
                    if token.get_type() == TokenType::Assignment {
                        tokens.read();
                        let child = Self::additive(tokens).unwrap();
                        node.add_children(child);
                        if let Some(token) = tokens.peek() {
                            if token.get_type() == TokenType::SemiColon {
                                tokens.read();
                                return Ok(Some(Rc::new(node)));
                            } else {
                                return Err(anyhow!("should be semicolon"));
                            }
                        } else {
                            return Err(anyhow!("lack semicolon"));
                        }
                    } else {
                        tokens.unread();
                    }
                }
            }
        }
        Ok(None)
    }

    // 整型变量声明
    // 如 int a;
    // 如 int a = 2 + 3;
    pub fn int_declare(mut tokens: &mut Box<dyn TokenReader>) -> ResultType<Option<Rc<dyn ASTNode>>> {
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
                                } else {
                                    return Err(anyhow!("should be additive"));
                                }

                                if let Some(token) = tokens.peek() {
                                    if token.get_type() == TokenType::SemiColon {
                                        tokens.read();
                                        return Ok(Some(Rc::new(node)));
                                    } else {
                                        return Err(anyhow!("should be semicolon"));
                                    }
                                } else {
                                    return Err(anyhow!("lack semicolon"));
                                }
                            }
                        } else {
                            return Ok(Some(Rc::new(node)));
                        }
                    } else {
                        return Err(anyhow!("should be identifier"));
                    }
                }
            }
        }
        Ok(None)
    }

    fn additive(tokens: &mut Box<dyn TokenReader>) -> Option<Rc<dyn ASTNode>> {
        let mut child1 = Self::multiplicative(tokens).unwrap();
        let mut node = child1.clone();
        loop {
            if let Some(token) = tokens.peek() {
                if token.get_type() == TokenType::Plus || token.get_type() == TokenType::Minus {
                    tokens.read();
                    let child2 = Self::multiplicative(tokens).unwrap();
                    let mut tmp_node = SimpleASTNode::new(ASTNodeType::Additive, token.get_text());
                    tmp_node.add_children(child1);
                    tmp_node.add_children(child2);
                    node = Rc::new(tmp_node);
                    child1 = node.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(node)
    }

    fn multiplicative(mut tokens: &mut Box<dyn TokenReader>) -> Option<Rc<dyn ASTNode>> {
        let mut child1 = Self::primary(tokens).unwrap();
        let mut node = child1.clone();
        loop {
            if let Some(token) = tokens.peek() {
                if token.get_type() == TokenType::Star || token.get_type() == TokenType::Slash {
                    tokens.read();
                    let child2 = Self::multiplicative(&mut tokens).unwrap();
                    let mut tmp_node = SimpleASTNode::new(ASTNodeType::Multiplicative, token.get_text());
                    tmp_node.add_children(child1);
                    tmp_node.add_children(child2);
                    node = Rc::new(tmp_node);
                    child1 = node.clone(); 
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(child1)
    }

    fn primary(tokens: &mut Box<dyn TokenReader>) -> Option<Rc<dyn ASTNode>> {
        if let Some(token) = tokens.peek() {
            if token.get_type() == TokenType::Identifier {
                tokens.read();
                return Some(Rc::new(SimpleASTNode::new(ASTNodeType::Identifier, token.get_text())));
            } else if token.get_type() == TokenType::IntLiteral {
                tokens.read();
                return Some(Rc::new(SimpleASTNode::new(ASTNodeType::IntLiteral, token.get_text())));
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

    pub fn dump_ast_tree(node: &Rc<dyn ASTNode>, ident: String) {
        println!("{}{:?} {:?}", ident, node.get_type(), node.get_text());
        for child in node.get_children() {
            let ident = ident.clone() + "\t";
            Self::dump_ast_tree(&child, ident);
        }
    }
}

pub struct SimpleASTNode {
    pub children: Vec<Rc<dyn ASTNode>>,
    pub node_type: ASTNodeType,
    pub node_text: String,
}

impl ASTNode for SimpleASTNode {
    fn get_children(&self) -> &Vec<Rc<dyn ASTNode>> {
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

    pub fn add_children(&mut self, child: Rc<dyn ASTNode>) {
        self.children.push(child)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let code = "a = 3 + 2 + 1;".to_owned();
        let tree = SimpleParser::parse(code);
        SimpleParser::dump_ast_tree(&tree, "".to_owned());
        println!("------------------");

        let code = "int a = 3 + 5 * 2;".to_owned();
        let tree = SimpleParser::parse(code);
        SimpleParser::dump_ast_tree(&tree, "".to_owned());
        println!("------------------");

        let code = "3 * 5;".to_owned();
        let tree = SimpleParser::parse(code);
        SimpleParser::dump_ast_tree(&tree, "".to_owned());
        println!("------------------");

        let code = "int age = 45 + 2; age = 20; age + 10 * 2;".to_owned();
        let tree = SimpleParser::parse(code);
        SimpleParser::dump_ast_tree(&tree, "".to_owned());
        println!("------------------");
    }
}

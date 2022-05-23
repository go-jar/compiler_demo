use std::rc::Rc;
use std::collections::HashMap;
use anyhow::anyhow;
use crate::{ASTNode, ASTNodeType, ResultType, simple_parser::SimpleParser};

pub struct SimpleScript {
    variables: HashMap<String, i32>,
}

impl SimpleScript {
    pub fn new() -> Self {
        SimpleScript {
            variables: HashMap::new(),
        }
    }

    pub fn calc(&mut self, node: &Rc<dyn ASTNode>, ident: String) -> ResultType<i32> {
        let mut result: i32 = 0;
        println!("{}Caculating: {:?}", ident.clone(), node.get_type());
        match node.get_type() {
            ASTNodeType::Programm => {
                if let Some(child) = node.get_children().get(0) {
                    result = self.calc(child, ident.clone() + "\t").unwrap();
                }
            }
            ASTNodeType::Additive => {
                let child1 = node.get_children().get(0).unwrap();
                let value1 = self.calc(child1, ident.clone() + "\t").unwrap();
                let child2 = node.get_children().get(1).unwrap();
                let value2 = self.calc(child2, ident.clone() + "\t").unwrap();
                if node.get_text() == "+" {
                    result = value1 + value2;
                } else {
                    result = value1 - value2;
                }
            }
            ASTNodeType::Multiplicative => {
                let child1 = node.get_children().get(0).unwrap();
                let value1 = self.calc(child1, ident.clone() + "\t").unwrap();
                let child2 = node.get_children().get(1).unwrap();
                let value2 = self.calc(child2, ident.clone() + "\t").unwrap();
                if node.get_text() == "*" {
                    result = value1 * value2;
                } else {
                    result = value1 / value2;
                }
            }
            ASTNodeType::IntLiteral => {
                result = node.get_text().parse::<i32>().unwrap();
            }
            ASTNodeType::Identifier => {
                let var_name = node.get_text();
                if let Some(v) = self.variables.get(&var_name) {
                    result = *v;
                } else {
                    return Err(anyhow!("invalid identifier"));
                }
            }
            ASTNodeType::AssignmentStmt => {
                let var_name = node.get_text();
                if let Some(_) = self.variables.get(&var_name) {
                } else {
                    return Err(anyhow!("invalid assignment"));
                }
            }
            ASTNodeType::IntDeclaration => {
                let var_name = node.get_text();
                result = 0;
                if node.get_children().len() > 0 {
                    let child = node.get_children().get(0).unwrap();
                    result = self.calc(child, ident.clone() + "\t").unwrap();
                }
                self.variables.insert(var_name, result);
            }
            _ => {}
        }
        println!("{}Result: {:?}", ident.clone(), result);
        Ok(result)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script() {
        let mut script = SimpleScript::new();

        let code = "int a = 3 + 2 + 1;".to_owned();
        let tree = SimpleParser::parse(code);
        script.calc(&tree, "".to_owned()).unwrap();
        println!("------------------");

        let code = "int b = a * 2;".to_owned();
        let tree = SimpleParser::parse(code);
        script.calc(&tree, "".to_owned()).unwrap();
        println!("------------------");
    }
}

use std::rc::Rc;
use anyhow;

pub mod simple_lexer;
pub mod simple_parser;
pub mod simple_script;

pub type ResultType<F, E = anyhow::Error> = anyhow::Result<F, E>;

pub trait TokenReader {
    fn read(&mut self) -> Option<&Box<dyn Token>>;
    fn peek(&self) -> Option<Box<dyn Token>>;
    fn unread(&mut self);
    fn get_position(&self) -> usize;
    fn set_position(&mut self, pos: usize);
}

pub trait Token {
    fn get_type(&self) -> TokenType;
    fn get_text(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    None,

    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /

    GE,     // >=
    GT,     // >
    EQ,     // ==
    LE,     // <=
    LT,     // <

    SemiColon, // ;
    LeftParen, // (
    RightParen,// )

    Assignment,// =

    If,
    Else,
    
    Int,

    Identifier,     //标识符

    IntLiteral,     //整型字面量
    StringLiteral   //字符串字面量
}

pub trait ASTNode {
    fn get_children(&self) -> &Vec<Rc<dyn ASTNode>>;
    fn get_type(&self) -> ASTNodeType;
    fn get_text(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum ASTNodeType{
    Programm,           //程序入口，根节点

    IntDeclaration,     //整型变量声明
    ExpressionStmt,     //表达式语句，即表达式后面跟个分号
    AssignmentStmt,     //赋值语句

    Primary,            //基础表达式
    Multiplicative,     //乘法表达式
    Additive,           //加法表达式

    Identifier,         //标识符
    IntLiteral          //整型字面量
}

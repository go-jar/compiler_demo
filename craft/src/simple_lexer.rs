use crate::{Token, TokenReader, TokenType};

// 词法分析器
pub struct SimpleLexer {
    tokens: Vec<Box<dyn Token>>,
}

impl SimpleLexer {
    pub fn new() -> Self {
        let tokens: Vec<Box<dyn Token>> = Vec::new(); 
        SimpleLexer {
            tokens,
        }
    }

    pub fn tokenize(mut self, code: String) -> Box<dyn TokenReader> {
        let mut token = SimpleToken::default();
        let mut state = DfaState::Initial;

        for c in code.chars() {
            match state {
                DfaState::Initial => {
                    let (state1, token1) = Self::init_token(c); // 重新确定后续状态
                    state = state1;
                    token = token1;
                }
                DfaState::Id => {
                    if is_alpha(c) || is_digit(c) {
                        token.token_text.push(c);  // 保持标识符状态
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);  // 退出标识符状态，并保存 token
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::IdInt1 => {
                    if c == 'n' {
                        token.token_text.push(c);
                        state = DfaState::IdInt2;
                    } else if is_alpha(c) || is_digit(c) {
                        token.token_text.push(c);
                        state = DfaState::Id;
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::IdInt2 => {
                    if c == 't' {
                        token.token_text.push(c);
                        state = DfaState::IdInt3;
                    } else if is_alpha(c) || is_digit(c) {
                        token.token_text.push(c);
                        state = DfaState::Id;
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::IdInt3 => {
                    if is_blank(c) {
                        token.token_type = TokenType::Int;
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::GT => {
                    if c == '=' {
                        state = DfaState::GE;
                        token.token_type = TokenType::GE;
                        token.token_text.push(c);
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::Assignment => {
                    if c == '=' {
                        state = DfaState::EQ;
                        token.token_type = TokenType::EQ;
                        token.token_text.push(c);
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::IntLiteral => {
                    if is_digit(c) {
                        token.token_text.push(c);
                    } else {
                        self.tokens.push(Box::new(token.clone()));
                        let (state1, token1) = Self::init_token(c);
                        state = state1;
                        token = token1;
                    }
                }
                DfaState::RightParen => {
                    self.tokens.push(Box::new(token.clone()));
                    let (state1, token1) = Self::init_token(c);
                    state = state1;
                    token = token1;
                }
                _ => {
                    self.tokens.push(Box::new(token.clone()));
                    let (state1, token1) = Self::init_token(c);
                    state = state1;
                    token = token1;
                }
            }
        }

        if token.token_text.len() > 0 {
            self.tokens.push(Box::new(token.clone()));
        }

        println!("tokens count: {:?}", self.tokens.len());
        Box::new(SimpleTokenReader::new(self.tokens))
    }

    pub fn init_token(c: char) -> (DfaState, SimpleToken) {
        let mut state = DfaState::Initial;
        let mut token = SimpleToken::default();

        match c {
            'i' => {
                state = DfaState::IdInt1;
                token.token_type = TokenType::Int;
            }
            '>' => {
                state = DfaState::GT;
                token.token_type = TokenType::GT;
            }
            '<' => {
                state = DfaState::LT;
                token.token_type = TokenType::LT;
            }
            '(' => {
                state = DfaState::LeftParen;
                token.token_type = TokenType::LeftParen;
            }
            ')' => {
                state = DfaState::RightParen;
                token.token_type = TokenType::RightParen;
            }
            ';' => {
                state = DfaState::SemiColon;
                token.token_type = TokenType::SemiColon;
            }
            '+' => {
                state = DfaState::Plus;
                token.token_type = TokenType::Plus;
            }
            '-' => {
                state = DfaState::Minus;
                token.token_type = TokenType::Minus;
            }
            '*' => {
                state = DfaState::Star;
                token.token_type = TokenType::Star;
            }
            '/' => {
                state = DfaState::Slash;
                token.token_type = TokenType::Slash;
            }
            '=' => {
                state = DfaState::Assignment;
                token.token_type = TokenType::Assignment;
            }
            _ => {
                if is_digit(c) {
                    state = DfaState::IntLiteral;
                    token.token_type = TokenType::IntLiteral;
                }
                if is_alpha(c) {
                    state = DfaState::Id;
                    token.token_type = TokenType::Identifier;
                }
            }
        }
        token.token_text.push(c);
        (state, token)
    }

    pub fn dump(tokens: &mut Box<dyn TokenReader>) {
        while let Some(token) = tokens.peek() {
            tokens.read();
            println!("{:?}\t\t{:?}", token.get_text(), token.get_type());
        }
    }
}

fn is_alpha(ch: char) -> bool {
    ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z' 
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn is_blank(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n'
}

#[derive(Debug, Clone)]
pub enum DfaState {
    Initial,

    If, IdIf1, IdIf2, Else, IdElse1, IdElse2, IdElse3, 
    IdElse4, Int, IdInt1, IdInt2, IdInt3, Id, GT, GE, LT, EQ,

    Assignment,

    Plus, Minus, Star, Slash,

    SemiColon,
    LeftParen,
    RightParen,

    IntLiteral,
}

#[derive(Clone)]
pub struct SimpleToken {
    pub token_type: TokenType,
    pub token_text: String,
}

impl Default for SimpleToken {
    fn default() -> Self {
        let s = String::new();
        SimpleToken {
            token_type: TokenType::Plus,
            token_text: s,
        }
    }
}

impl Token for SimpleToken {
    fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }

    fn get_text(&self) -> String {
        self.token_text.clone()
    }
}

pub struct SimpleTokenReader {
    pos: usize,
    tokens: Vec<Box<dyn Token>>,
}

impl SimpleTokenReader {
    pub fn new(tokens: Vec<Box<dyn Token>>) -> Self {
        SimpleTokenReader {
            pos: 0,
            tokens,
        }
    }
}

impl TokenReader for SimpleTokenReader {
    fn read(&mut self) -> Option<&Box<dyn Token>> {
        let token = self.tokens.get(self.pos);
        self.pos = self.pos + 1;
        token
    }

    fn peek(&self) -> Option<Box<dyn Token>> {
        if self.pos > self.tokens.len() - 1 {
            return None;
        }
        let token = &self.tokens[self.pos];
        Some(Box::new(SimpleToken{
            token_type: token.get_type().clone(),
            token_text: token.get_text().clone(),
        }))
    }

    fn unread(&mut self) {
        if self.pos > 1 {
            self.pos = self.pos - 1;
        }
    }

    fn get_position(&self) -> usize {
        self.pos
    }

    fn set_position(&mut self, pos: usize) {
        if pos < self.tokens.len() {
            self.pos = pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let lexer = SimpleLexer::new();
        let code = "int a = b + 1;".to_owned();
        let mut tokens = lexer.tokenize(code);
        SimpleLexer::dump(&mut tokens);
        println!("------------------");

        let lexer = SimpleLexer::new();
        let code = "int a = 1;".to_owned();
        let mut tokens = lexer.tokenize(code);
        SimpleLexer::dump(&mut tokens);
        println!("------------------");

        let lexer = SimpleLexer::new();
        let code = "int a = (bb + 1) + (45 + 5);".to_owned();
        let mut tokens = lexer.tokenize(code);
        SimpleLexer::dump(&mut tokens);
        println!("------------------");

        let lexer = SimpleLexer::new();
        let code = "int a = 2 + 3 + 4;".to_owned();
        let mut tokens = lexer.tokenize(code);
        SimpleLexer::dump(&mut tokens);
        println!("------------------");
    }
}

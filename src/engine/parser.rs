use super::token::{Token, TokenWithPos};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    PostfixOp {
        op: PostfixOp,
        expr: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Pos,
    Neg,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOp {
    Factorial, // '!'
    Percent,   // '%'
}

pub struct Parser {
    tokens: Vec<TokenWithPos>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPos>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current).map(|t| &t.token)
    }

    fn peek_pos(&self) -> usize {
        self.tokens.get(self.current).map(|t| t.pos).unwrap_or(0)
    }

    fn advance(&mut self) -> Option<TokenWithPos> {
        if self.current < self.tokens.len() {
            let t = self.tokens[self.current].clone();
            self.current += 1;
            Some(t)
        } else {
            None
        }
    }

    fn check(&self, expected: &Token) -> bool {
        self.peek() == Some(expected)
    }

    fn match_token(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        let expr = self.parse_expression(0)?;
        if self.current < self.tokens.len() {
            let unexpected = &self.tokens[self.current];
            return Err(format!(
                "Unexpected token {:?} at position {}",
                unexpected.token, unexpected.pos
            ));
        }
        Ok(expr)
    }

    // Pratt parser / Precedence climbing
    fn parse_expression(&mut self, min_bp: u8) -> Result<Expr, String> {
        let mut lhs = self.parse_prefix()?;

        // Postfix operators loop (e.g. 5!, 20%)
        while let Some(token) = self.peek() {
            match token {
                Token::Bang => {
                    self.advance();
                    lhs = Expr::PostfixOp {
                        op: PostfixOp::Factorial,
                        expr: Box::new(lhs),
                    };
                }
                Token::Percent => {
                    // In standard scientific calc, % right after a number/expression is percent (val / 100)
                    // If followed by an expression, e.g. 10 % 3, it acts as modulo binary op.
                    // We disambiguate: if the next token is an operator or RParen or end-of-input, it's postfix percent.
                    // Otherwise, it's treated as modulo binary op.
                    let is_postfix = if self.current + 1 >= self.tokens.len() {
                        true
                    } else {
                        matches!(
                            self.tokens[self.current + 1].token,
                            Token::Plus
                                | Token::Minus
                                | Token::Star
                                | Token::Slash
                                | Token::RParen
                                | Token::Comma
                                | Token::Caret
                                | Token::Bang
                        )
                    };

                    if is_postfix {
                        self.advance();
                        lhs = Expr::PostfixOp {
                            op: PostfixOp::Percent,
                            expr: Box::new(lhs),
                        };
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        // Infix operators
        loop {
            let (op, l_bp, r_bp) = match self.peek() {
                Some(Token::Plus) => (BinaryOp::Add, 10, 11),
                Some(Token::Minus) => (BinaryOp::Sub, 10, 11),
                Some(Token::Star) => (BinaryOp::Mul, 20, 21),
                Some(Token::Slash) => (BinaryOp::Div, 20, 21),
                Some(Token::Percent) => (BinaryOp::Mod, 20, 21),
                Some(Token::Caret) => (BinaryOp::Pow, 31, 30), // Right-associative: 31 > 30
                _ => break,
            };

            if l_bp < min_bp {
                break;
            }

            self.advance(); // Consume operator
            let rhs = self.parse_expression(r_bp)?;
            lhs = Expr::BinaryOp {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, String> {
        let pos = self.peek_pos();
        match self.peek() {
            Some(Token::Plus) => {
                self.advance();
                let expr = self.parse_expression(25)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Pos,
                    expr: Box::new(expr),
                })
            }
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_expression(25)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            Some(Token::Number(val)) => {
                let v = *val;
                self.advance();
                Ok(Expr::Number(v))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression(0)?;
                if !self.match_token(&Token::RParen) {
                    return Err(format!("Missing closing parenthesis ')' matching opening at position {}", pos));
                }
                Ok(expr)
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();

                // If followed by '(', it's a function call
                if self.match_token(&Token::LParen) {
                    let mut args = Vec::new();
                    if !self.check(&Token::RParen) {
                        loop {
                            args.push(self.parse_expression(0)?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    if !self.match_token(&Token::RParen) {
                        return Err(format!("Expected ')' after function arguments for '{}' at position {}", name, pos));
                    }
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    // Constant or variable (e.g. pi, e, ans)
                    Ok(Expr::Variable(name))
                }
            }
            Some(tok) => Err(format!("Unexpected token {:?} at position {}", tok, pos)),
            None => Err(format!("Unexpected end of expression at position {}", pos)),
        }
    }
}


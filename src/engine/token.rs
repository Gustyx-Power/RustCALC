#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Percent,
    Bang, // Factorial '!'
    LParen,
    RParen,
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithPos {
    pub token: Token,
    pub pos: usize,
}

pub struct Lexer<'a> {
    _phantom: std::marker::PhantomData<&'a str>,
    chars: Vec<(usize, char)>,
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            chars: input.char_indices().collect(),
            idx: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.idx).map(|&(_, c)| c)
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        if self.idx < self.chars.len() {
            let res = self.chars[self.idx];
            self.idx += 1;
            Some(res)
        } else {
            None
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<TokenWithPos>, String> {
        let mut raw_tokens: Vec<TokenWithPos> = Vec::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            let (pos, _) = self.chars[self.idx];

            match c {
                '+' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Plus, pos });
                }
                '-' | '−' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Minus, pos });
                }
                '*' | '×' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Star, pos });
                }
                '/' | '÷' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Slash, pos });
                }
                '^' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Caret, pos });
                }
                '%' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Percent, pos });
                }
                '!' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Bang, pos });
                }
                '(' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::LParen, pos });
                }
                ')' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::RParen, pos });
                }
                ',' => {
                    self.advance();
                    raw_tokens.push(TokenWithPos { token: Token::Comma, pos });
                }
                '0'..='9' | '.' => {
                    let num_tok = self.lex_number(pos)?;
                    raw_tokens.push(num_tok);
                }
                'a'..='z' | 'A'..='Z' | '_' | 'π' | 'τ' | 'φ' => {
                    let ident_tok = self.lex_ident(pos)?;
                    raw_tokens.push(ident_tok);
                }
                other => {
                    return Err(format!("Unexpected character '{}' at position {}", other, pos));
                }
            }
        }

        // Insert implicit multiplication where applicable
        // e.g.:
        //   Number Ident -> Number * Ident (e.g. 2pi)
        //   Number LParen -> Number * LParen (e.g. 2(3+4))
        //   RParen Number -> RParen * Number (e.g. (2)3)
        //   RParen LParen -> RParen * LParen (e.g. (2)(3))
        //   RParen Ident -> RParen * Ident (e.g. (2)pi)
        //   Ident(const) Number -> Ident * Number
        //   Ident(const) Ident -> Ident * Ident
        //   Ident(const) LParen -> Ident * LParen
        //   Bang Number / Bang Ident / Bang LParen -> Bang * ...
        let expanded = self.insert_implicit_multiplication(raw_tokens);
        Ok(expanded)
    }

    fn lex_number(&mut self, start_pos: usize) -> Result<TokenWithPos, String> {
        let mut num_str = String::new();
        let mut has_dot = false;
        let mut has_exp = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.advance();
            } else if c == ',' {
                // If followed by a digit, it's a thousand separator (e.g. 1,000)
                if self.idx + 1 < self.chars.len() && self.chars[self.idx + 1].1.is_ascii_digit() {
                    self.advance(); // skip comma
                } else {
                    break;
                }
            } else if c == '.' && !has_dot && !has_exp {
                has_dot = true;
                num_str.push(c);
                self.advance();
            } else if (c == 'e' || c == 'E') && !has_exp {
                // Lookahead to check if this is scientific notation or identifier
                // Scientific notation must be preceded by at least one digit or dot
                has_exp = true;
                num_str.push(c);
                self.advance();

                if let Some(next) = self.peek()
                    && (next == '+' || next == '-')
                {
                    num_str.push(next);
                    self.advance();
                }
            } else {
                break;
            }
        }

        match num_str.parse::<f64>() {
            Ok(val) => Ok(TokenWithPos {
                token: Token::Number(val),
                pos: start_pos,
            }),
            Err(_) => Err(format!("Invalid number literal '{}' at position {}", num_str, start_pos)),
        }
    }

    fn lex_ident(&mut self, start_pos: usize) -> Result<TokenWithPos, String> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == 'π' || c == 'τ' || c == 'φ' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Normalize unicode symbols to standard identifier names
        let normalized = match s.as_str() {
            "π" => "pi".to_string(),
            "τ" => "tau".to_string(),
            "φ" => "phi".to_string(),
            _ => s.to_lowercase(),
        };

        Ok(TokenWithPos {
            token: Token::Ident(normalized),
            pos: start_pos,
        })
    }

    fn insert_implicit_multiplication(&self, tokens: Vec<TokenWithPos>) -> Vec<TokenWithPos> {
        if tokens.is_empty() {
            return tokens;
        }

        let is_constant = |name: &str| -> bool {
            matches!(name, "pi" | "e" | "phi" | "tau" | "ans" | "c" | "h" | "g" | "na" | "k")
        };

        let mut result = Vec::with_capacity(tokens.len() * 2);

        for i in 0..tokens.len() {
            result.push(tokens[i].clone());

            if i + 1 < tokens.len() {
                let curr = &tokens[i].token;
                let next = &tokens[i + 1].token;
                let next_pos = tokens[i + 1].pos;

                let should_insert = match (curr, next) {
                    // 2 pi, 2 sin(x) -> 2 * pi, 2 * sin(x)
                    (Token::Number(_), Token::Ident(_)) => true,
                    // 2 (x) -> 2 * (x)
                    (Token::Number(_), Token::LParen) => true,
                    // ) ( -> ) * (
                    (Token::RParen, Token::LParen) => true,
                    // ) 2 -> ) * 2
                    (Token::RParen, Token::Number(_)) => true,
                    // ) pi, ) sin(x) -> ) * pi, ) * sin(x)
                    (Token::RParen, Token::Ident(_)) => true,
                    // 5! 2 -> 5! * 2, 5! (x) -> 5! * (x)
                    (Token::Bang, Token::Number(_)) => true,
                    (Token::Bang, Token::LParen) => true,
                    (Token::Bang, Token::Ident(_)) => true,
                    // constant (e.g. pi) followed by number, paren, or another ident
                    (Token::Ident(name), Token::LParen) if is_constant(name) => true,
                    (Token::Ident(name), Token::Number(_)) if is_constant(name) => true,
                    (Token::Ident(name), Token::Ident(_)) if is_constant(name) => true,
                    _ => false,
                };

                if should_insert {
                    result.push(TokenWithPos {
                        token: Token::Star,
                        pos: next_pos,
                    });
                }
            }
        }

        result
    }
}

use crate::parser::lexer::{Lexer, LexerError, TokenPos};

use crate::util;

#[derive(Debug)]
pub enum LangTokenType {
    Name,
    LiteralString,
    LiteralNumber,
    LiteralTrue,
    LiteralFalse,
    LiteralNull,

    ObjectBegin,
    ObjectEnd,
    ArrayBegin,
    ArrayEnd,

    GroupBegin,
    GroupEnd,

    Comma,
    Dot,
    Colon,

    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
pub struct LangToken<'a> {
    token_type: LangTokenType,
    text: &'a str, // 'a is lifetime of source code string (tokens point into that)
    pos: TokenPos,
}

impl<'a> LangToken<'a> {
    pub fn new(token_type: LangTokenType, text: &'a str, pos: TokenPos) -> LangToken<'a> {
        LangToken {
            token_type, text, pos
        }
    }

    pub fn token_type(&self) -> &LangTokenType { &self.token_type }
    pub fn text(&self) -> &str { &self.text }
    pub fn pos(&self) -> &TokenPos { &self.pos }
}

pub struct LangLexer<'a> {
    lexer: Lexer<'a>,
}

impl<'a> LangLexer<'a> {
    pub fn new(source: &'a str) -> LangLexer<'a> {
        LangLexer {
            lexer: Lexer::new(source)
        }
    }

    fn make_token(&self, token_type: LangTokenType) -> LangToken {
        LangToken::new(token_type, self.lexer.get_token_text(), self.lexer.pos())
    }

    pub fn scan_token(&mut self) -> Result<LangToken, LexerError> {
        self.lexer.skip_whitespace();
        self.lexer.set_start_pos_to_current();
        let c = self.lexer.consume()?;

        match c {
            '{' => Ok(self.make_token(LangTokenType::ObjectBegin)),
            '}' => Ok(self.make_token(LangTokenType::ObjectEnd)),
            '[' => Ok(self.make_token(LangTokenType::ArrayBegin)),
            ']' => Ok(self.make_token(LangTokenType::ArrayEnd)),
            '(' => Ok(self.make_token(LangTokenType::GroupBegin)),
            ')' => Ok(self.make_token(LangTokenType::GroupEnd)),
            ',' => Ok(self.make_token(LangTokenType::Comma)),
            ':' => Ok(self.make_token(LangTokenType::Colon)),
            '.' => Ok(self.make_token(LangTokenType::Dot)),

            '+' => Ok(self.make_token(LangTokenType::Plus)),
            '-' if !util::is_digit(*self.lexer.peek()?) => Ok(self.make_token(LangTokenType::Minus)),
            '*' => Ok(self.make_token(LangTokenType::Star)),
            '/' => Ok(self.make_token(LangTokenType::Slash)),

            '"' => {
                self.lexer.set_start_pos_to_current(); // Don't include leading '"'

                while *self.lexer.peek()? != '"' {
                    let _ = self.lexer.consume();
                }

                self.lexer.consume()?; // the trailing '"'
                let text = self.lexer.get_token_text();

                Ok(LangToken::new(LangTokenType::LiteralString, &text[.. text.len() - 1], self.lexer.pos()))
            },

            '0'..='9' | '-' => {
                while util::is_digit(*self.lexer.peek()?) {
                    let _ = self.lexer.consume();
                }

                if let Ok(_) = self.lexer.expect('.') {
                    while util::is_digit(*self.lexer.peek()?) {
                        let _ = self.lexer.consume();
                    }
                }

                let next = *self.lexer.peek()?;

                if next == 'e' || next == 'E' { // Exponent
                    let _ = self.lexer.consume();
                    let next = *self.lexer.peek()?;

                    if next == '+' || next == '-' {
                        let _ = self.lexer.consume();
                    }

                    let next = *self.lexer.peek()?;

                    if !util::is_digit(next) {
                        return Err(LexerError::UnexpectedCharacter(self.lexer.current_pos, next));
                    }

                    while util::is_digit(*self.lexer.peek()?) {
                        let _ = self.lexer.consume();
                    }
                }

                Ok(self.make_token(LangTokenType::LiteralNumber))
            },

            't' => {
                self.lexer.expect_str("rue")?;
                Ok(self.make_token(LangTokenType::LiteralTrue))
            },
            'f' => {
                self.lexer.expect_str("alse")?;
                Ok(self.make_token(LangTokenType::LiteralFalse))
            },
            'n' => {
                self.lexer.expect_str("ull")?;
                Ok(self.make_token(LangTokenType::LiteralNull))
            },
            _ if util::is_alpha(c) => {
                while util::is_alpha_numeric(*self.lexer.peek()?) {
                    let _ = self.lexer.consume();
                }

                Ok(self.make_token(LangTokenType::Name))
            }

            _ => Err(LexerError::UnexpectedCharacter(self.lexer.pos(), c))
        }
    }
}

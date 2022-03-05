use crate::parser::lexer::{Lexer, LexerError, TokenPos};

use crate::util;

#[derive(PartialEq, Clone, Debug)]
pub enum LangTokenType {
    None,

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

    Eof,
}

#[derive(Clone, Debug)]
pub struct LangToken {
    token_type: LangTokenType,
    text: String, // Can't be a reference to the source because self-referential structs aren't safe
    pos: TokenPos,
}

impl<'a> LangToken {
    pub fn new(token_type: LangTokenType, text: String, pos: TokenPos) -> LangToken {
        LangToken {
            token_type, text, pos
        }
    }

    pub fn empty() -> LangToken {
        LangToken {
            token_type: LangTokenType::None,
            text: String::from(""),
            pos: TokenPos::begin()
        }
    }

    pub fn token_type(&self) -> &LangTokenType { &self.token_type }
    pub fn text(&self) -> &str { &self.text }
    pub fn pos(&self) -> &TokenPos { &self.pos }
}

pub struct LangLexer<'a> {
    lexer: Lexer<'a>,
}

type LexerResult<T> = Result<T, LexerError>;

impl<'a> LangLexer<'a> {
    pub fn new(source: &'a str) -> LangLexer<'a> {
        LangLexer {
            lexer: Lexer::new(source)
        }
    }

    fn make_token(&self, token_type: LangTokenType) -> LangToken {
        // Need to copy the String from the source because Token can't store a reference
        // (Tokens need to be stored alongside the source string later, which would be
        // self-referential)
        LangToken::new(token_type, String::from(self.lexer.get_token_text()), self.lexer.pos())
    }

    pub fn scan_token(&mut self) -> LexerResult<LangToken> {
        self.lexer.skip_whitespace();
        self.lexer.set_start_pos_to_current();
        let c = match self.lexer.consume() {
            result @ Ok(_) => result,
            Err(LexerError::UnexpectedEof) => return Ok(LangToken::new(LangTokenType::Eof, String::from(""), self.lexer.pos())),
             result @ Err(_) => result
        }?;

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

                Ok(LangToken::new(LangTokenType::LiteralString, String::from(&text[.. text.len() - 1]), self.lexer.pos()))
            },

            '0'..='9' | '-' => {
                while util::is_digit(*self.lexer.peek()?) {
                    let _ = self.lexer.consume();
                }

                if let Ok(_) = self.lexer.expect('.') {
                    let mut c = *self.lexer.peek()?;

                    if !util::is_digit(c) {
                        return Err(LexerError::UnexpectedCharacter(self.lexer.current_pos, c));
                    }

                    while util::is_digit(c) {
                        let _ = self.lexer.consume();
                        c = *self.lexer.peek()?;
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

impl<'a> Iterator for LangLexer<'a> {
    type Item = LexerResult<LangToken>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.scan_token() {
            Ok(token) if *token.token_type() == LangTokenType::Eof => None,
            result => Some(result),
        }
    }
}

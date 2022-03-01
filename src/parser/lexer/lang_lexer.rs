use crate::parser::lexer::{Lexer, LexerError, TokenPos};

#[derive(Debug)]
pub enum LangTokenType {
    Name,
    LiteralString,
    LiteralNumber,

    ObjectBegin,
    ObjectEnd,
    ArrayBegin,
    ArrayEnd,
    FieldEnd,

    GroupBegin,
    GroupEnd,
    Dot,

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
        LangToken::new(token_type, self.lexer.get_token_text(), self.lexer.pos)
    }

    pub fn scan_token(&mut self) -> Result<LangToken, LexerError> {
        self.lexer.set_start_pos_to_current();
        let c = self.lexer.consume()?;

        match c {
            '{' => Ok(self.make_token(LangTokenType::ObjectBegin)),
            '}' => Ok(self.make_token(LangTokenType::ObjectEnd)),
            '[' => Ok(self.make_token(LangTokenType::ArrayBegin)),
            ']' => Ok(self.make_token(LangTokenType::ArrayEnd)),
            ',' => Ok(self.make_token(LangTokenType::FieldEnd)),
            '(' => Ok(self.make_token(LangTokenType::GroupBegin)),
            ')' => Ok(self.make_token(LangTokenType::GroupEnd)),
            '.' => Ok(self.make_token(LangTokenType::Dot)),

            '+' => Ok(self.make_token(LangTokenType::Plus)),
            '-' => Ok(self.make_token(LangTokenType::Minus)),
            '*' => Ok(self.make_token(LangTokenType::Star)),
            '/' => Ok(self.make_token(LangTokenType::Slash)),

            _ => Err(LexerError::UnexpectedCharacter(self.lexer.pos(), c))
        }
    }
}

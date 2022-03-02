use std::iter::Peekable;
use crate::element::Element;
use crate::parser::lexer::{LangLexer, LangToken, LangTokenType, LexerError, TokenPos};
use crate::util;
use crate::util::EscapeError;

#[derive(Debug)]
pub enum ParseError<'a> {
    UnexpectedEof,

    UnexpectedCharacter(TokenPos, char),
    ExpectedCharacter {
        pos: TokenPos,
        expected: char,
        got: char
    },

    UnexpectedToken(TokenPos, LangToken<'a>),
    ExpectedToken {
        pos: TokenPos,
        expected: LangTokenType,
        got: LangToken<'a>
    },

    OtherError(TokenPos, String)
}

impl<'a> From<LexerError> for ParseError<'a> { // To use ? operator on lexer methods
    fn from(err: LexerError) -> Self {
        match err {
            LexerError::UnexpectedEof => ParseError::UnexpectedEof,
            LexerError::UnexpectedCharacter(pos, c) => ParseError::UnexpectedCharacter(pos, c),
            LexerError::ExpectedCharacter { pos, expected, got } => ParseError::ExpectedCharacter { pos, expected, got },

            LexerError::OtherError(pos, message) => ParseError::OtherError(pos, message)
        }
    }
}

impl<'a> ParseError<'a> {
    pub fn from_escape_error(e: EscapeError, pos: TokenPos) -> Self {
        match e {
            EscapeError::UnexpectedEof => ParseError::UnexpectedEof,
            EscapeError::UnexpectedCharacter(pos_i, c) => ParseError::UnexpectedCharacter(
                TokenPos::new(pos.line, pos.column + pos_i), c),
            EscapeError::FailedConversion(err) => ParseError::OtherError(pos, format!("Failed conversion to u32: {}", err.to_string())),
            EscapeError::InvalidCharacter(err) => ParseError::OtherError(pos, format!("Invalid character: {}", err.to_string())),
        }
    }
}

pub struct LangParser<'a> {
    lexer: Peekable<LangLexer<'a>>,

    // Can't store Tokens here
    // previous: LangToken<'a>, current: LangToken<'a>,
}

type ParseResult<'e, T> = Result<T, ParseError<'e>>;

impl<'a> LangParser<'a> {
    pub fn new(lexer: LangLexer<'a>) -> Self {
        let parser = LangParser {
            lexer: lexer.peekable(),
        };

        // Sets current to first token (parse_element assumes this to be the case)
        // Ignores any error; will be handled by the next call to parse_element
        // let _ = parser.consume();
        parser
    }

    fn consume(&mut self) -> ParseResult<'_, LangToken<'_>> {
        // std::mem::swap(&mut self.previous, &mut self.current); // self.previous = self.current; cannot move
        // self.current = self.lexer.scan_token()?; // Set current to next token;
        Ok(self.lexer.next().ok_or(ParseError::UnexpectedEof)??)
    }

    fn expect(&mut self, token_type: LangTokenType) -> ParseResult<'_, LangToken<'_>> {
        let next = match self.lexer.peek().ok_or(ParseError::UnexpectedEof)? {
            Ok(token) => token,
            Err(_) => return self.consume(), // If an error occurred, it should be fine to consume the next token
        };

        if *next.token_type() == token_type {
            self.consume()
        } else {
            // Err(ParseError::ExpectedToken { pos: *next.pos(), expected: token_type, got: *next })
            todo!()
        }
    }

    fn parse_number(token: &LangToken<'_>) -> ParseResult<'a, Element> {
        token.text().parse::<i32>()
            .map(|value| Element::IntElement(value))
            .or_else(|_err| token.text().parse::<f64>()
                .map(|value| Element::FloatElement(value)))
            .map_err(|err| ParseError::OtherError(*token.pos(),
                format!("Float parse error: {}", err.to_string())))
    }

    pub fn parse_element(&mut self) -> ParseResult<'_, Element> {
        let token = self.consume()?;

        match token.token_type() {
            LangTokenType::Name => Ok(Element::NameElement(String::from(token.text()))),
            LangTokenType::LiteralString => Ok(Element::StringElement(util::unescape_str(token.text())
                .map_err(|err| ParseError::from_escape_error(err, *token.pos()))?)),
            LangTokenType::LiteralNumber => LangParser::parse_number(&token),
            LangTokenType::LiteralTrue => Ok(Element::BooleanElement(true)),
            LangTokenType::LiteralFalse => Ok(Element::BooleanElement(false)),
            LangTokenType::LiteralNull => Ok(Element::NullElement),
            LangTokenType::ObjectBegin => todo!(),
            LangTokenType::ArrayBegin => todo!(),
            LangTokenType::GroupBegin => todo!(),

            LangTokenType::Eof => Err(ParseError::UnexpectedEof),
            _ => Err(ParseError::UnexpectedToken(*token.pos(), token.clone())),
        }
    }
}

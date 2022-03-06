use std::fmt::{Display, Formatter};

use crate::element::Element;
use crate::parser::lexer::{LangLexer, LangToken, LangTokenType, LexerError, TokenPos};
use crate::util;
use crate::util::EscapeError;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,

    UnexpectedCharacter(TokenPos, char),
    ExpectedCharacter {
        pos: TokenPos,
        expected: char,
        got: char
    },

    UnexpectedToken(TokenPos, LangToken),
    ExpectedToken {
        pos: TokenPos,
        expected: LangTokenType,
        got: LangToken
    },

    OtherError(TokenPos, String)
}

impl From<LexerError> for ParseError { // To use ? operator on lexer methods
    fn from(err: LexerError) -> Self {
        match err {
            LexerError::UnexpectedEof => ParseError::UnexpectedEof,
            LexerError::UnexpectedCharacter(pos, c) => ParseError::UnexpectedCharacter(pos, c),
            LexerError::ExpectedCharacter { pos, expected, got } => ParseError::ExpectedCharacter { pos, expected, got },

            LexerError::OtherError(pos, message) => ParseError::OtherError(pos, message)
        }
    }
}

impl ParseError {
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEof => f.write_str("Unexpected EOF"),
            ParseError::UnexpectedCharacter(pos, c) => write!(f, "{0} Unexpected character: '{1}'", pos, c),
            ParseError::ExpectedCharacter { pos, expected, got } =>
                write!(f, "{0} Expected character '{1}', got: '{2}'", pos, expected, got),
            ParseError::UnexpectedToken(pos, token) =>
                write!(f, "{0} Unexpected token: {1}", pos, token),
            ParseError::ExpectedToken { pos, expected, got } =>
                write!(f, "{0} Expected token type {1:?}, got: {2}", pos, expected, got),
            ParseError::OtherError(pos, message) => write!(f, "{0} Other error: {1}", pos, message),
        }
    }
}

pub struct LangParser<'a> {
    lexer: LangLexer<'a>,

    previous: LangToken, current: LangToken, // This is the reason LangToken can't store a reference
}

type ParseResult<T> = Result<T, ParseError>;

impl<'a> LangParser<'a> {
    pub fn new(lexer: LangLexer<'a>) -> Self {
        let mut parser = LangParser {
            lexer,

            previous: LangToken::empty(), current: LangToken::empty()
        };

        // Sets current to first token (parse_element assumes this to be the case)
        // Ignores any error; will be handled by the next call to parse_element
        let _ = parser.consume();
        parser
    }

    fn peek(&self) -> &LangToken {
        &self.current
    }

    fn is_eof(&self) -> bool {
        self.current.token_type() == LangTokenType::Eof
    }

    fn consume(&mut self) -> ParseResult<()> {
        std::mem::swap(&mut self.previous, &mut self.current); // self.previous = self.current; cannot move
        self.current = self.lexer.scan_token()?; // Set current to next token;

        Ok(())
    }

    fn expect(&mut self, token_type: LangTokenType) -> ParseResult<&LangToken> {
        if self.current.token_type() == token_type {
            self.consume()?;
            Ok(&self.current)
        } else {
            // Clone should be fine in the error case
            Err(ParseError::ExpectedToken { pos: *self.current.pos(), expected: token_type, got: self.current.clone() })
        }
    }

    fn parse_number(token: &LangToken) -> ParseResult<Element> {
        token.text().parse::<i32>()
            .map(|value| Element::IntElement(value))
            .or_else(|_err| token.text().parse::<f64>()
                .map(|value| Element::FloatElement(value)))
            .map_err(|err| ParseError::OtherError(*token.pos(),
                format!("Float parse error: {}", err.to_string())))
    }

    fn parse_object(&mut self) -> ParseResult<Element> {
        // Can use self.previous to see the object begin token ('{')

        let mut fields = Vec::new();

        while self.peek().token_type() != LangTokenType::ObjectEnd && !self.is_eof() {
            let key = self.parse_element()?;
            self.expect(LangTokenType::Colon)?;
            let value = self.parse_element()?;

            fields.push((key, value));

            let comma = self.expect(LangTokenType::Comma).map(|x| x.clone());

            if self.peek().token_type() != LangTokenType::ObjectEnd {
                comma?; // If the next token is not ')', this will report an error if the comma is missing
            }
        }

        let _ = self.consume(); // ObjectEnd
        Ok(Element::ObjectElement(fields))
    }

    fn parse_array(&mut self) -> ParseResult<Element> {
        let mut elements = Vec::new();

        while self.peek().token_type() != LangTokenType::ArrayEnd && !self.is_eof() {
            elements.push(self.parse_element()?);

            let comma = self.expect(LangTokenType::Comma).map(|x| x.clone());

            if self.peek().token_type() != LangTokenType::ArrayEnd {
                comma?; // If the next token is not ')', this will report an error if the comma is missing
            }
        }

        let _ = self.consume(); // ArrayEnd
        Ok(Element::ArrayElement(elements))
    }

    fn parse_group(&mut self) -> ParseResult<Element> {
        let element = self.parse_element()?;
        self.expect(LangTokenType::GroupEnd)?;

        Ok(element)
    }

    pub fn parse_element(&mut self) -> ParseResult<Element> {
        self.consume()?;

        match self.previous.token_type() {
            LangTokenType::Name => Ok(Element::NameElement(String::from(self.previous.text()))),
            LangTokenType::LiteralString => Ok(Element::StringElement(util::unescape_str(self.previous.text())
                .map_err(|err| ParseError::from_escape_error(err, *self.previous.pos()))?)),
            LangTokenType::LiteralNumber => LangParser::parse_number(&self.previous),
            LangTokenType::LiteralTrue => Ok(Element::BooleanElement(true)),
            LangTokenType::LiteralFalse => Ok(Element::BooleanElement(false)),
            LangTokenType::LiteralNull => Ok(Element::NullElement),
            LangTokenType::ObjectBegin => self.parse_object(),
            LangTokenType::ArrayBegin => self.parse_array(),
            LangTokenType::GroupBegin => self.parse_group(),

            LangTokenType::Eof => Err(ParseError::UnexpectedEof),
            _ => Err(ParseError::UnexpectedToken(*self.previous.pos(), self.previous.clone())),
        }
    }
}

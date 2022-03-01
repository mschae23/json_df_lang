pub mod lang_lexer;

use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Copy, Debug)]
pub struct TokenPos {
    pub line: i32,
    pub column: i32,
}

pub enum LexerError {
    UnexpectedEof,

    UnexpectedCharacter(TokenPos, char),
    ExpectedCharacter {
        pos: TokenPos,
        expected: char,
        got: char
    }
}

impl TokenPos {
    pub fn new(line: i32, column: i32) -> TokenPos {
        TokenPos { line, column }
    }

    pub fn begin() -> TokenPos {
        TokenPos::new(0, 0)
    }
}

struct Lexer<'a> {
    source: &'a str,
    source_chars: Peekable<Chars<'a>>,
    start: usize, current: usize,
    pos: TokenPos,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source, source_chars: source.chars().peekable(),
            start: 0, current: 0,
            pos: TokenPos::begin()
        }
    }

    pub fn pos(&self) -> TokenPos { self.pos }

    pub fn peek(&mut self) -> Result<&char, LexerError> {
        self.source_chars.peek().ok_or(LexerError::UnexpectedEof)
    }

    pub fn consume(&mut self) -> Result<char, LexerError> {
        self.source_chars.next().ok_or(LexerError::UnexpectedEof).map(|c| {
            if c == '\n' {
                self.pos.line += 1;
                self.pos.column = 0;
            } else {
                self.pos.column += 1;
            }

            c
        })
    }

    pub fn expect(&mut self, expected: char) -> Result<char, LexerError> {
        let pos = self.pos;
        let c = self.peek()?;

        if expected == *c {
            self.consume()
        } else {
            Err(LexerError::ExpectedCharacter {
                pos,
                expected,
                got: *c
            })
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Ok(c) = self.peek() {
            if !c.is_whitespace() {
                return;
            }

            let _ = self.consume();
        }
    }

    pub fn skip_line(&mut self) {
        while let Ok(c) = self.peek() {
            if *c == '\n' {
                return;
            }

            let _ = self.consume();
        }
    }

    // Expects the starting '/' of the comment to already be matched
    pub fn skip_comment(&mut self) -> Result<(), LexerError> {
        if let Ok(_) = self.expect('/') {
            self.skip_line();
            Ok(())
        } else if let Ok(_) = self.expect('*') {
            let mut comment_count = 1;

            loop {
                let c = self.consume()?;

                match c {
                    '/' => { // Allow nested comments
                        if let Ok(_) = self.expect('*') {
                            comment_count += 1;
                        }
                    },
                    '*' => {
                        if let Ok(_) = self.expect('/') {
                            comment_count -= 1;

                            if comment_count <= 0 {
                                return Ok(());
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else {
            Err(LexerError::UnexpectedCharacter(self.pos, *self.peek()?))
        }
    }

    pub fn skip(&mut self, number: i32) -> Result<(), LexerError> {
        for _ in [0..number].iter() {
            self.consume()?;
        }

        Ok(())
    }

    pub fn expect_str<'b>(&mut self, expected: &'b str) -> Result<&'b str, LexerError> {
        for c in expected.chars() {
            self.expect(c)?;
        }

        Ok(expected)
    }

    pub fn get_token_text(&self) -> &'a str {
        &self.source[self.start..self.current]
    }

    pub fn set_start_pos_to_current(&mut self) {
        self.start = self.current;
    }
}

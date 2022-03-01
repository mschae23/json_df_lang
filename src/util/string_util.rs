use std::char::DecodeUtf16Error;
use std::cmp;
use std::num::ParseIntError;

#[inline]
pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[inline]
pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '$' || c == '_'
}

#[inline]
pub fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

pub enum EscapeError {
    UnexpectedEof,
    UnexpectedCharacter(i32, char),
    FailedConversion(ParseIntError),
    InvalidCharacter(DecodeUtf16Error),
}

pub fn unescape_character(input: &str) -> Result<char, EscapeError> {
    let mut chars = input.chars();
    let c = chars.next().ok_or(EscapeError::UnexpectedEof)?;

    match c {
        '"' => Ok('"'),
        '\\' => Ok('\\'),
        '/' => Ok('/'),
        // 'b' => Ok('\b'),
        // 'f' => Ok('\f'),
        'n' => Ok('\n'),
        'r' => Ok('\r'),
        't' => Ok('\t'),
        'u' => {
            let mut string = String::new();

            for i in 0..4 {
                let c = chars.next().ok_or(EscapeError::UnexpectedEof)?;

                if c.is_digit(16) {
                    string.push(c);
                } else {
                    return Err(EscapeError::UnexpectedCharacter(1 + i, c));
                }
            }

            char::decode_utf16([u16::from_str_radix(&string, 16)
                .map_err(|err| EscapeError::FailedConversion(err))?]).next()
                .ok_or_else(|| EscapeError::UnexpectedEof)?
                .map_err(|err| EscapeError::InvalidCharacter(err))
        }

        _ => Err(EscapeError::UnexpectedCharacter(0, c))
    }
}

pub fn escape_character(c: char) -> String {
    match c {
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        // '\b' => "\\\b".to_string(),
        // '\f' => "\\\f".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\x20'..='\x7e' => c.to_string(),
        _ => {
            let string = format!("{:x}", c as u16);
            let mut result = String::with_capacity(6);
            result.push_str("\\u");
            result.push_str(&"0".repeat(cmp::max(0, 4 - (string.len() as i32)) as usize));
            result.push_str(&string);

            result
        }
    }
}

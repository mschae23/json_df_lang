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

pub fn unescape_str(input: &str) -> Result<String, EscapeError> {
    if input.is_empty() {
        return Ok(String::from(""));
    }

    let mut output = String::new();
    let mut chars = input.chars();

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => return Ok(output),
        };

        if c != '\\' {
            output.push(c);
            continue;
        }

        let c = match chars.next() {
            Some(c) => c,
            None => return Err(EscapeError::UnexpectedEof),
        };

        match c {
            '"' => output.push('"'),
            '\\' => output.push('\\'),
            '/' => output.push('/'),
            // 'b' => output.push('\b'),
            // 'f' => output.push('\f'),
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
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

                output.push(char::decode_utf16([u16::from_str_radix(&string, 16)
                    .map_err(|err| EscapeError::FailedConversion(err))?]).next()
                    .ok_or_else(|| EscapeError::UnexpectedEof)?
                    .map_err(|err| EscapeError::InvalidCharacter(err))?);
            }

            _ => return Err(EscapeError::UnexpectedCharacter(0, c)), // TODO error pos
        }
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

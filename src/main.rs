use worldgen_lang::parser::lexer::{LangLexer, LexerError};

fn main() {
    let source = r#"
{ "enabled": true, "example_factor": 5.3, "fields": [ { "type": "minecraft:something", value: 99 }, 20 + 3 * 2 ] }
    "#;

    let mut lexer = LangLexer::new(source);

    loop {
        match lexer.scan_token() {
            Ok(token) => println!("{:?}", token),
            Err(LexerError::UnexpectedEof) => break,
            Err(e) => {
                println!("Error: {:?}", e);
                break
            },
        }
    }

    println!("Done.");
}

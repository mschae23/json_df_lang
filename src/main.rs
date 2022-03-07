use worldgen_lang::parser::LangParser;
use worldgen_lang::parser::lexer::LangLexer;

fn main() {
    let source = r#"
{ "enabled": true, "example_factor": 5.3, "fields": [ { "type": "minecraft:something", value: 99 }, 20 + 3 * min(9, 3 + 5) ] }
    "#;

    let _source = r#" [ 384.1, "Hello\tworld\u0021" ] "#;

    let lexer = LangLexer::new(source); // Moved into parser
    let mut parser = LangParser::new(lexer);

    match parser.parse_element() {
        Ok(element) => println!("{:?}", element),
        Err(err) => println!("Error while parsing: {}", err),
    }

    /*
    loop {
        match lexer.scan_token() {
            Ok(token) if *token.token_type() == LangTokenType::Eof => break,
            Ok(token) => println!("{:?}", token),
            Err(e) => {
                println!("Error: {:?}", e);
                break
            },
        }
    }
    */

    println!("Done.");
}

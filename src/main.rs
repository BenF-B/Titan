mod scanner;

use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/bfb/titan/src/example.titan").unwrap();

    let mut context = scanner::Context {
        file_name: "example.titan".to_string(),
        file_length: contents.len(),
        file_pointer: 0,
        file_end: contents.len(),
        buffer: contents,
        tokens: Vec::new(),
    };

    scanner::scan_tokens(&mut context);

    for token in context.tokens.iter() {
        println!("{:?}", token.token_type);
    }
}

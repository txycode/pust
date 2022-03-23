use pust::{lexer::Lexer, parser::Parser};

pub fn run<'a>(f: String, text: String) {
    let mut lexer = Lexer::new(f, text);
    let r = lexer.make_tokens();
    match r {
        Ok((tokens, posctxs)) => {
            let mut parsor = Parser {
                tokens: &tokens,
                posctxs: &posctxs,
                index: 0,
            };
            let ast = parsor.parse();
            println!("{:?}", ast);
            // println!("{:?}", tokens);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

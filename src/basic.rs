use pust::{interpreter::Interpreter, lexer::Lexer, parser::Parser, symbol_table::SymbolTable};

pub fn init() -> Interpreter {
    Interpreter {
        st: SymbolTable::new(),
    }
}

pub fn run<'a>(f: String, text: String, interpreter: &'a mut Interpreter) {
    let mut lexer = Lexer::new(f, text);

    let r = lexer.make_tokens();
    match r {
        Ok((tokens, posctxs)) => {
            let mut parsor = Parser {
                tokens: &tokens,
                posctxs: &posctxs,
                index: 0,
            };
            // println!("{:?}", tokens);
            let ast = parsor.parse();
            // println!("{:?}", ast);
            let v = match ast {
                Ok(node) => {
                    // println!("{:?}", node);
                    interpreter.evaluate(node)
                }
                Err(error) => Err(error), // Err(error) => println!("{:?}", error),
            };
            match v {
                Ok(value) => {
                    println!("{:?}", value);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

use num_bigint::BigInt;

#[derive(Debug)]
pub enum Token {
    Int { value: BigInt },
    Float { value: f64 },
    Plus,
    Minus,
    Mul,
    Div,
    Lpar,
    Rpar,
}

#[derive(Debug)]
pub struct ErrorCtx<'a> {
    pub pos_start: Position<'a>,
    pub pos_end: Position<'a>,
    pub cur_char: char,
}

#[derive(Debug)]
pub enum LexerError<'a> {
    IllegalCharError { ctx: ErrorCtx<'a>, detail: &'a str },
}

#[derive(Clone, Debug)]
pub struct Position<'a> {
    pub idx: u128,
    pub col: u128,
    pub row: u128,
    pub fd: &'a str,
    pub text: &'a str,
    pub len: u128,
}
pub struct Lexer<'a> {
    pub pos: Position<'a>,
    pub cur_char: char,
}

impl Lexer<'_> {
    pub fn make_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];
        self.advance();
        while self.cur_char != '\0' {
            match self.cur_char {
                ' ' | '\t' => {
                    self.advance();
                }
                '0'..='9' => tokens.push(self.make_numbers()),
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance()
                }
                '-' => {
                    tokens.push(Token::Plus);
                    self.advance()
                }
                '*' => {
                    tokens.push(Token::Mul);
                    self.advance()
                }
                '/' => {
                    tokens.push(Token::Div);
                    self.advance()
                }
                '(' => {
                    tokens.push(Token::Lpar);
                    self.advance()
                }
                ')' => {
                    tokens.push(Token::Rpar);
                    self.advance()
                }
                _ => {
                    let pos_start = self.pos.clone();
                    let char = self.cur_char;
                    self.advance();
                    let ctx = ErrorCtx {
                        pos_start,
                        pos_end: self.pos.clone(),
                        cur_char: char,
                    };
                    let detail = "";
                    return Err(LexerError::IllegalCharError { ctx, detail });
                }
            }
        }
        Ok(tokens)
    }
    pub fn make_numbers(&mut self) -> Token {
        let mut num_chars: Vec<char> = vec![];

        let mut dot_count = 0u32;

        while self.cur_char != '\0' {
            match self.cur_char {
                '.' => {
                    if dot_count == 1 {
                        break;
                    } else {
                        dot_count += 1;
                        num_chars.push('.');
                    }
                }
                '0'..='9' => {
                    num_chars.push(self.cur_char);
                }
                _ => {
                    break;
                }
            }
            self.advance()
        }
        let num_str = String::from_iter(num_chars);
        if dot_count == 0 {
            Token::Int {
                value: num_str.parse().unwrap(),
            }
        } else {
            Token::Float {
                value: num_str.parse().unwrap(),
            }
        }
    }
    pub fn advance(&mut self) {
        self.pos.advance(self.cur_char);
        if self.pos.idx < self.pos.len {
            self.cur_char = self
                .pos
                .text
                .chars()
                .nth((self.pos.idx - 1) as usize)
                .unwrap();
        } else {
            self.cur_char = '\0';
        }
    }
}

impl Position<'_> {
    pub fn advance(&mut self, cur_char: char) {
        self.idx += 1;
        self.col += 1;
        if cur_char == '\n' {
            self.col = 0;
            self.row += 1;
        }
    }
}

pub fn build_lexer<'a>(fd: &'a str, text: &'a str) -> Lexer<'a> {
    let position = Position {
        idx: 0,
        col: 0,
        row: 0,
        fd,
        text,
        len: (text.chars().count() + 1) as u128,
    };
    let lexer = Lexer {
        pos: position,
        cur_char: '\0',
    };
    lexer
}
pub fn run<'a>(f: &'a str, text: &'a str) -> Vec<Token> {
    let mut lexer = build_lexer(f, text);
    let r = lexer.make_tokens();
    match r {
        Ok(tokens) => {
            println!("{:?}", tokens);
            return tokens;
        }
        Err(error) => {
            println!("{:?}", error);
            vec![]
        }
    }
}

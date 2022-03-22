use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub enum Token {
    Int { value: BigInt },
    Float { value: f64 },
    Plus { value: char },
    Minus { value: char },
    Mul,
    Div,
    Lpar,
    Rpar,
    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct PosCtx {
    pub pos_start: Box<Position>,
    pub pos_end: Box<Position>,
}

#[derive(Debug)]
pub enum Error {
    IllegalChar { ctx: PosCtx, detail: String },
    InvalidSyntax { ctx: Option<PosCtx>, detail: String },
}

#[derive(Clone, Debug)]
pub struct Position {
    pub idx: usize,
    pub col: u128,
    pub row: u128,
    pub fd: String,
    pub text: String,
    pub len: usize,
}

impl Position {
    pub fn advance(&mut self, cur_char: char) {
        self.idx += 1;
        self.col += 1;
        if cur_char == '\n' {
            self.col = 0;
            self.row += 1;
        }
    }
}
pub struct Lexer {
    pub pos: Box<Position>,
    pub cur_char: char,
}

#[derive(Debug)]
pub enum ASTNode {
    Number {
        token: Token,
    },
    BinOp {
        token: Token,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Unary {
        token: Token,
        node: Box<ASTNode>,
    },
}

pub struct Parser<'a> {
    tokens: &'a Vec<(Token, PosCtx)>,
    current_token: Token,
    current_posctx: PosCtx,
    index: usize,
}

type ParseResult = Result<Box<ASTNode>, Error>;

impl<'a> Parser<'a> {
    pub fn advance(&mut self) {
        self.index += 1;
        self.current_token = self.tokens[self.index - 1].0.clone();
        self.current_posctx = self.tokens[self.index - 1].1.clone();
    }

    pub fn parse(&mut self) -> ParseResult {
        self.advance();
        self.expr()
    }

    pub fn expr(&mut self) -> ParseResult {
        let mut left = self.term();
        if let Err(error) = left {
            Err(error)
        } else {
            loop {
                match self.current_token {
                    // Token::EndOfFile => {
                    //     break;
                    // }
                    Token::Plus { value: _ } | Token::Minus { value: _ } => {
                        let tk = self.current_token.clone();
                        self.advance();
                        let right = self.term();
                        if let Err(error) = right {
                            return Err(error);
                        } else {
                            left = Ok(Box::new(ASTNode::BinOp {
                                token: tk,
                                left: left.unwrap(),
                                right: right.unwrap(),
                            }));
                        }
                    }
                    _ => {
                        break;
                        // return Err(Error::InvalidSyntax {
                        //     ctx: Some(self.current_posctx.clone()),
                        //     detail: String::from(format!(
                        //         "Got: '{:?}', Expect: '+', '-'",
                        //         self.current_token
                        //     )),
                        // })
                    }
                }
            }
            left
        }
    }
    pub fn term(&mut self) -> ParseResult {
        let mut left = self.factor();
        if let Err(error) = left {
            Err(error)
        } else {
            loop {
                match self.current_token {
                    // Token::EndOfFile => {
                    //     break;
                    // }
                    Token::Mul | Token::Div => {
                        let tk = self.current_token.clone();
                        self.advance();
                        let right = self.factor();
                        if let Err(error) = right {
                            return Err(error);
                        } else {
                            left = Ok(Box::new(ASTNode::BinOp {
                                token: tk,
                                left: left.unwrap(),
                                right: right.unwrap(),
                            }));
                        }
                    }
                    _ => {
                        break;
                        // return Err(Error::InvalidSyntax {
                        //     ctx: Some(self.current_posctx.clone()),
                        //     detail: String::from(format!(
                        //         "Got: '{:?}', Expect: '*', '/'",
                        //         self.current_token
                        //     )),
                        // })
                    }
                }
            }
            left
        }
    }

    pub fn factor(&mut self) -> ParseResult {
        match self.current_token {
            // None => {
            //     return Err(Error::InvalidSyntax {
            //         ctx: None,
            //         detail: String::from(format!(
            //             "Missing Token: 'None', Expect '+', '-', '(', 'float', 'int'"
            //         )),
            //     })
            // }
            Token::Int { value: _ } | Token::Float { value: _ } => {
                let r = Ok(Box::new(ASTNode::Number {
                    token: self.current_token.clone(),
                }));
                self.advance();
                r
            }
            Token::Plus { value: _ } | Token::Minus { value: _ } => {
                self.advance();
                let parse_result = self.factor();
                if let Ok(node) = parse_result {
                    Ok(Box::new(ASTNode::Unary {
                        token: self.current_token.clone(),
                        node,
                    }))
                } else {
                    parse_result
                }
            }
            Token::Lpar => {
                self.advance();
                let parse_result = self.expr();
                if let Ok(node) = parse_result {
                    match self.current_token {
                        Token::Rpar => {
                            self.advance();
                            return Ok(node);
                        }
                        _ => {
                            return Err(Error::InvalidSyntax {
                                ctx: Some(self.current_posctx.clone()),
                                detail: String::from(format!(
                                    "Got: '{:?}', Expect: ')'",
                                    self.current_token
                                )),
                            })
                        }
                    }
                } else {
                    return parse_result;
                }
            }
            _ => {
                return Err(Error::InvalidSyntax {
                    ctx: Some(self.current_posctx.clone()),
                    detail: String::from(format!(
                        "Got: '{:?}', Expect: '(', '+', '-', 'float', 'int'",
                        self.current_token
                    )),
                })
            }
        }
    }
}

impl Lexer {
    pub fn make_tokens(&mut self) -> Result<Vec<(Token, PosCtx)>, Box<Error>> {
        let mut tokens: Vec<(Token, PosCtx)> = vec![];
        self.advance();
        // let mut ctx = Box::new(PosCtx {
        //     pos_start: self.pos.clone(),
        //     pos_end: self.pos.clone(),
        // });
        while self.cur_char != '\0' {
            let cur_pos = self.pos.clone();
            match self.cur_char {
                ' ' | '\t' => {
                    self.advance();
                }
                '0'..='9' => tokens.push(self.make_numbers()),
                '+' => {
                    tokens.push((
                        Token::Plus { value: '+' },
                        PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        },
                    ));
                    self.advance();
                }
                '-' => {
                    tokens.push((
                        Token::Minus { value: '-' },
                        PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        },
                    ));
                    self.advance();
                }
                '*' => {
                    tokens.push((
                        Token::Mul,
                        PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        },
                    ));
                    self.advance();
                }
                '/' => {
                    tokens.push((
                        Token::Div,
                        PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        },
                    ));
                    self.advance();
                }
                '(' => {
                    tokens.push((
                        Token::Lpar,
                        PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        },
                    ));
                    self.advance();
                }
                ')' => {
                    tokens.push((
                        Token::Rpar,
                        PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        },
                    ));
                    self.advance();
                }
                _ => {
                    let pos_start = self.pos.clone();
                    self.advance();
                    return Err(Box::new(Error::IllegalChar {
                        ctx: PosCtx {
                            pos_start,
                            pos_end: self.pos.clone(),
                        },
                        detail: format!("unknown character {}", self.cur_char),
                    }));
                }
            }
        }
        tokens.push((
            Token::EndOfFile,
            PosCtx {
                pos_start: self.pos.clone(),
                pos_end: self.pos.clone(),
            },
        ));
        Ok(tokens)
    }
    pub fn make_numbers(&mut self) -> (Token, PosCtx) {
        let mut num_chars: Vec<char> = vec![];

        let mut dot_count = 0u32;

        let cur_pos = self.pos.clone();
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
            (
                Token::Int {
                    value: num_str.parse().unwrap(),
                },
                PosCtx {
                    pos_start: cur_pos,
                    pos_end: self.pos.clone(),
                },
            )
        } else {
            (
                Token::Float {
                    value: num_str.parse().unwrap(),
                },
                PosCtx {
                    pos_start: cur_pos,
                    pos_end: self.pos.clone(),
                },
            )
        }
    }
    pub fn advance(&mut self) {
        self.pos.advance(self.cur_char);
        if self.pos.idx <= self.pos.len {
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

    pub fn new(fd: String, text: String) -> Box<Self> {
        let len = text.chars().count();
        Box::new(Lexer {
            pos: Box::new(Position {
                idx: 0,
                col: 0,
                row: 0,
                fd,
                text,
                len,
            }),
            cur_char: '\0',
        })
    }
}

pub fn run<'a>(f: String, text: String) -> Vec<(Token, PosCtx)> {
    let mut lexer = Lexer::new(f, text);
    let r = lexer.make_tokens();
    match r {
        Ok(tokens) => {
            let mut parsor = Parser {
                tokens: &tokens,
                current_token: Token::EndOfFile,
                current_posctx: tokens[0].1.clone(),
                index: 0,
            };
            let ast = parsor.parse();
            println!("{:?}", ast);
            // println!("{:?}", tokens);
            return tokens;
        }
        Err(error) => {
            println!("{:?}", error);
            vec![]
        }
    }
}

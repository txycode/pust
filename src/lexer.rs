use crate::{
    error::Error,
    position::{PosCtx, Position},
    token::Token,
};

pub struct Lexer {
    pub pos: Box<Position>,
    pub cur_char: char,
}

impl Lexer {
    pub fn make_tokens(&mut self) -> Result<(Vec<Box<Token>>, Vec<Box<PosCtx>>), Box<Error>> {
        let mut tokens: Vec<Box<Token>> = vec![];
        let mut posctxs: Vec<Box<PosCtx>> = vec![];
        self.advance(1);
        while self.cur_char != '\0' {
            let cur_pos = self.pos.clone();
            match self.cur_char {
                ' ' | '\t' => {
                    self.advance(1);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let (token, postctx) = self.make_idents();
                    tokens.push(Box::new(token));
                    posctxs.push(Box::new(postctx));
                }
                '0'..='9' => {
                    let (token, postctx) = self.make_numbers();
                    tokens.push(Box::new(token));
                    posctxs.push(Box::new(postctx));
                }
                '+' => {
                    tokens.push(Box::new(Token::Plus { value: '+' }));
                    posctxs.push(Box::new(PosCtx {
                        pos_start: cur_pos.clone(),
                        pos_end: cur_pos,
                    }));
                    self.advance(1);
                }
                '-' => {
                    tokens.push(Box::new(Token::Minus { value: '-' }));
                    posctxs.push(Box::new(PosCtx {
                        pos_start: cur_pos.clone(),
                        pos_end: cur_pos,
                    }));
                    self.advance(1);
                }
                '*' => {
                    let next_char = self.peek();
                    if next_char == '*' {
                        tokens.push(Box::new(Token::Pow {
                            value: "**".to_string(),
                        }));
                        self.advance(2);
                        posctxs.push(Box::new(PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: self.pos.clone(),
                        }));
                    } else {
                        tokens.push(Box::new(Token::Mul { value: '*' }));
                        posctxs.push(Box::new(PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: cur_pos,
                        }));
                        self.advance(1);
                    }
                }
                '/' => {
                    tokens.push(Box::new(Token::Div { value: '/' }));
                    posctxs.push(Box::new(PosCtx {
                        pos_start: cur_pos.clone(),
                        pos_end: cur_pos,
                    }));
                    self.advance(1);
                }
                '=' => {
                    let (token, postctx) = self.make_equality();
                    tokens.push(Box::new(token));
                    posctxs.push(Box::new(postctx));
                }
                '(' => {
                    tokens.push(Box::new(Token::Lpar { value: '(' }));
                    posctxs.push(Box::new(PosCtx {
                        pos_start: cur_pos.clone(),
                        pos_end: cur_pos,
                    }));
                    self.advance(1);
                }
                ')' => {
                    tokens.push(Box::new(Token::Rpar { value: ')' }));
                    posctxs.push(Box::new(PosCtx {
                        pos_start: cur_pos.clone(),
                        pos_end: cur_pos,
                    }));
                    self.advance(1);
                }
                '>' | '<' => {
                    let (token, postctx) = self.make_comparison();
                    tokens.push(Box::new(token));
                    posctxs.push(Box::new(postctx));
                }
                '!' => {
                    let next_char = self.peek();
                    if next_char == '=' {
                        tokens.push(Box::new(Token::NotEq {
                            value: "!=".to_string(),
                        }));
                        self.advance(2);
                        posctxs.push(Box::new(PosCtx {
                            pos_start: cur_pos.clone(),
                            pos_end: self.pos.clone(),
                        }));
                    } else {
                        return Err(Box::new(Error::IllegalChar {
                            ctx: Box::new(PosCtx {
                                pos_start,
                                pos_end: self.pos.clone(),
                            }),
                            detail: format!("unknown character '!'"),
                        }));
                    }
                }
                _ => {
                    let pos_start = self.pos.clone();
                    self.advance(1);
                    return Err(Box::new(Error::IllegalChar {
                        ctx: Box::new(PosCtx {
                            pos_start,
                            pos_end: self.pos.clone(),
                        }),
                        detail: format!("unknown character {}", self.cur_char),
                    }));
                }
            }
        }
        tokens.push(Box::new(Token::EndOfFile));

        posctxs.push(Box::new(PosCtx {
            pos_start: self.pos.clone(),
            pos_end: self.pos.clone(),
        }));
        Ok((tokens, posctxs))
    }
    fn make_numbers(&mut self) -> (Token, PosCtx) {
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
            self.advance(1)
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

    fn make_idents(&mut self) -> (Token, PosCtx) {
        let mut ident = String::new();
        let cur_pos = self.pos.clone();
        while (self.cur_char.is_alphanumeric() || self.cur_char == '_') && self.cur_char != '\0' {
            ident.push(self.cur_char);
            self.advance(1);
        }
        (
            Token::Ident { value: ident },
            PosCtx {
                pos_start: cur_pos,
                pos_end: self.pos.clone(),
            },
        )
    }

    pub fn make_equality(&mut self) -> (Token, PosCtx) {
        let cur_pos = self.pos.clone();
        self.advance(1);
        if self.cur_char == '=' {
            self.advance(1);
            (
                Token::Eq {
                    value: "==".to_string(),
                },
                PosCtx {
                    pos_start: cur_pos,
                    pos_end: self.pos.clone(),
                },
            )
        } else {
            (
                Token::Assign { value: '=' },
                PosCtx {
                    pos_start: cur_pos,
                    pos_end: self.pos.clone(),
                },
            )
        }
    }

    pub fn make_comparison(&mut self) -> (Token, PosCtx) {
        let cur_pos = self.pos.clone();
        let op = self.cur_char;
        self.advance(1);
        if op == '>' {
            if self.cur_char == '=' {
                self.advance(1);
                return (
                    Token::GreaterEq {
                        value: ">=".to_string(),
                    },
                    PosCtx {
                        pos_start: cur_pos,
                        pos_end: self.pos.clone(),
                    },
                );
            } else {
                return (
                    Token::Greater { value: '>' },
                    PosCtx {
                        pos_start: cur_pos,
                        pos_end: self.pos.clone(),
                    },
                );
            }
        }
        else {
                if self.cur_char == '=' {
                    self.advance(1);
                    return (
                        Token::LessEq {
                            value: "<=".to_string(),
                        },
                        PosCtx {
                            pos_start: cur_pos,
                            pos_end: self.pos.clone(),
                        },
                    );
                } else {
                    return (
                        Token::Less { value: '<' },
                        PosCtx {
                            pos_start: cur_pos,
                            pos_end: self.pos.clone(),
                        },
                    );
                }
            }
        }
    }

    pub fn advance(&mut self, mut step: u128) {
        while step > 0 {
            self.pos.advance(self.cur_char);
            if self.pos.idx <= self.pos.len {
                self.cur_char = self.pos.text.chars().nth(self.pos.idx - 1).unwrap();
            } else {
                self.cur_char = '\0';
            }
            step -= 1;
        }
    }

    pub fn peek(&mut self) -> char {
        if self.pos.idx < self.pos.len {
            self.pos.text.chars().nth(self.pos.idx).unwrap()
        } else {
            '\0'
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

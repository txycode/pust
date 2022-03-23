use crate::{ast::ASTNode, error::Error, position::PosCtx, token::Token};

pub struct Parser<'a> {
    pub tokens: &'a Vec<Box<Token>>,
    pub posctxs: &'a Vec<Box<PosCtx>>,
    pub index: usize,
}

type ParseResult = Result<Box<ASTNode>, Error>;

impl<'a> Parser<'a> {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn get_current_token(&self) -> Box<Token> {
        return self.tokens[self.index - 1].clone();
    }

    fn get_current_posctx(&self) -> Box<PosCtx> {
        return self.posctxs[self.index - 1].clone();
    }

    pub fn parse(&mut self) -> ParseResult {
        self.advance();
        self.expr()
    }

    fn expr(&mut self) -> ParseResult {
        let mut left = self.term();
        if let Err(error) = left {
            Err(error)
        } else {
            loop {
                let tk = self.get_current_token();
                match *tk {
                    // Token::EndOfFile => {
                    //     break;
                    // }
                    Token::Plus { value: _ } | Token::Minus { value: _ } => {
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
                let tk = self.get_current_token();
                match *tk {
                    // Token::EndOfFile => {
                    //     break;
                    // }
                    Token::Mul { value: _ } | Token::Div { value: _ } => {
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
        let tk = self.get_current_token();
        match *tk {
            // None => {
            //     return Err(Error::InvalidSyntax {
            //         ctx: None,
            //         detail: String::from(format!(
            //             "Missing Token: 'None', Expect '+', '-', '(', 'float', 'int'"
            //         )),
            //     })
            // }
            Token::Int { value: _ } | Token::Float { value: _ } => {
                let r = Ok(Box::new(ASTNode::Number { token: tk }));
                self.advance();
                r
            }
            Token::Plus { value: _ } | Token::Minus { value: _ } => {
                self.advance();
                let next_tk = self.get_current_token();
                let parse_result = self.factor();
                if let Ok(node) = parse_result {
                    Ok(Box::new(ASTNode::Unary {
                        token: next_tk,
                        node,
                    }))
                } else {
                    parse_result
                }
            }
            Token::Lpar { value: _ } => {
                self.advance();
                let parse_result = self.expr();
                let next_tk = self.get_current_token();
                if let Ok(node) = parse_result {
                    match *next_tk {
                        Token::Rpar { value: _ } => {
                            self.advance();
                            return Ok(node);
                        }
                        _ => {
                            return Err(Error::InvalidSyntax {
                                ctx: self.get_current_posctx(),
                                detail: String::from(format!("Got: '{:?}', Expect: ')'", next_tk)),
                            })
                        }
                    }
                } else {
                    return parse_result;
                }
            }
            _ => {
                return Err(Error::InvalidSyntax {
                    ctx: self.get_current_posctx(),
                    detail: String::from(format!(
                        "Got: '{:?}', Expect: '(', '+', '-', 'float', 'int'",
                        tk
                    )),
                })
            }
        }
    }
}

use crate::{ast::ASTNode, error::Error, position::PosCtx, token::Token};

pub struct Parser<'a> {
    pub tokens: &'a Vec<Box<Token>>,
    pub posctxs: &'a Vec<Box<PosCtx>>,
    pub index: usize,
}

type ParseResult = Result<Box<ASTNode>, Box<Error>>;

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
                let token = self.get_current_token();
                let posctx = self.get_current_posctx();
                match *token {
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
                                token,
                                posctx,
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
                let token = self.get_current_token();
                let posctx = self.get_current_posctx();
                match *token {
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
                                token,
                                posctx,
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
        let token = self.get_current_token();
        let posctx = self.get_current_posctx();
        match *token {
            // None => {
            //     return Err(Error::InvalidSyntax {
            //         ctx: None,
            //         detail: String::from(format!(
            //             "Missing Token: 'None', Expect '+', '-', '(', 'float', 'int'"
            //         )),
            //     })
            // }
            Token::Int { value: _ } | Token::Float { value: _ } => {
                let r = Ok(Box::new(ASTNode::Number { token, posctx }));
                self.advance();
                r
            }
            Token::Plus { value: _ } | Token::Minus { value: _ } => {
                let current_token = self.get_current_token();
                self.advance();
                let posctx = self.get_current_posctx();
                let parse_result = self.factor();
                if let Ok(node) = parse_result {
                    Ok(Box::new(ASTNode::Unary {
                        token: current_token,
                        posctx,
                        node,
                    }))
                } else {
                    parse_result
                }
            }
            Token::Lpar { value: _ } => {
                self.advance();
                let parse_result = self.expr();
                let current_token = self.get_current_token();
                if let Ok(node) = parse_result {
                    match *current_token {
                        Token::Rpar { value: _ } => {
                            self.advance();
                            return Ok(node);
                        }
                        _ => Err(Box::new(Error::InvalidSyntax {
                            ctx: self.get_current_posctx(),
                            detail: String::from(format!(
                                "Got: '{:?}', Expect: ')'",
                                current_token
                            )),
                        })),
                    }
                } else {
                    return parse_result;
                }
            }
            _ => Err(Box::new(Error::InvalidSyntax {
                ctx: self.get_current_posctx(),
                detail: String::from(format!(
                    "Got: '{:?}', Expect: '(', '+', '-', 'float', 'int'",
                    token
                )),
            })),
        }
    }
}

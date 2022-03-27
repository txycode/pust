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

    fn peek(&mut self) -> Box<Token> {
        if self.index < self.tokens.len() {
            self.tokens[self.index].clone()
        } else {
            Token::EndOfFile.into()
        }
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
        let token = self.get_current_token();
        let posctx = self.get_current_posctx();

        match *token {
            Token::Ident { value: _ } => match *self.peek() {
                Token::Assign { value: _ } => {
                    self.advance();
                    self.advance();
                    let value_node = self.expr()?;
                    return Ok(Box::new(ASTNode::Assign {
                        token: token,
                        posctx: posctx.merge(&self.get_current_posctx()),
                        value: value_node,
                    }));
                }
                _ => {
                    let mut left = self.term()?;
                    loop {
                        let token = self.get_current_token();
                        let posctx = self.get_current_posctx();
                        match *token {
                            Token::Plus { value: _ } | Token::Minus { value: _ } => {
                                self.advance();
                                let right = self.term()?;

                                left = Box::new(ASTNode::BinOp {
                                    token,
                                    posctx,
                                    left: left,
                                    right: right,
                                });
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    Ok(left)
                }
            },
            _ => {
                let mut left = self.term()?;
                loop {
                    let token = self.get_current_token();
                    let posctx = self.get_current_posctx();
                    match *token {
                        Token::Plus { value: _ } | Token::Minus { value: _ } => {
                            self.advance();
                            let right = self.term()?;

                            left = Box::new(ASTNode::BinOp {
                                token,
                                posctx,
                                left: left,
                                right: right,
                            });
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Ok(left)
            }
        }
    }
    pub fn term(&mut self) -> ParseResult {
        let mut left = self.factor()?;

        loop {
            let token = self.get_current_token();
            let posctx = self.get_current_posctx();
            match *token {
                Token::Mul { value: _ } | Token::Div { value: _ } => {
                    self.advance();
                    let right = self.factor()?;
                    left = Box::new(ASTNode::BinOp {
                        token,
                        posctx,
                        left: left,
                        right: right,
                    });
                }
                _ => {
                    break;
                }
            }
        }
        Ok(left)
    }

    pub fn factor(&mut self) -> ParseResult {
        let token = self.get_current_token();
        match *token {
            Token::Plus { value: _ } | Token::Minus { value: _ } => {
                let current_token = self.get_current_token();
                self.advance();
                let posctx = self.get_current_posctx();
                let node = self.factor()?;
                Ok(Box::new(ASTNode::Unary {
                    token: current_token,
                    posctx,
                    node,
                }))
            }
            _ => self.power(),
        }
    }
    pub fn power(&mut self) -> ParseResult {
        let mut left = self.atom()?;
        loop {
            let token = self.get_current_token();
            let posctx = self.get_current_posctx();
            match *token {
                Token::Pow { value: _ } => {
                    self.advance();
                    let right = self.factor()?;
                    left = Box::new(ASTNode::BinOp {
                        token,
                        posctx,
                        left: left,
                        right: right,
                    });
                }
                _ => {
                    break;
                }
            }
        }
        Ok(left)
    }

    pub fn atom(&mut self) -> ParseResult {
        let token = self.get_current_token();
        let posctx = self.get_current_posctx();
        match *token {
            Token::Ident { value: _ } => {
                self.advance();
                Ok(Box::new(ASTNode::Access { token, posctx }))
            }
            Token::Int { value: _ } | Token::Float { value: _ } => {
                self.advance();
                Ok(Box::new(ASTNode::Number { token, posctx }))
            }
            Token::Lpar { value: _ } => {
                self.advance();
                let node = self.expr()?;
                let current_token = self.get_current_token();
                match *current_token {
                    Token::Rpar { value: _ } => {
                        self.advance();
                        return Ok(node);
                    }
                    _ => Err(Box::new(Error::InvalidSyntax {
                        ctx: self.get_current_posctx(),
                        detail: String::from(format!("Got: '{:?}', Expect: ')'", current_token)),
                    })),
                }
            }
            _ => Err(Box::new(Error::InvalidSyntax {
                ctx: self.get_current_posctx(),
                detail: String::from(format!(
                    "Got: '{:?}', Expect: 'ident', 'int', 'float', '('",
                    token
                )),
            })),
        }
    }
}

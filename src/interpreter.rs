use crate::ast::ASTNode;
use crate::error::Error;
use crate::position::PosCtx;
use crate::token::Token;
use num_bigint::{BigInt, ToBigInt};
use num_traits::cast::ToPrimitive;

pub struct Interpreter;
#[derive(Debug)]
pub enum NumValue {
    BigInt { value: BigInt, ctx: Box<PosCtx> },
    Float { value: f64, ctx: Box<PosCtx> },
}

type RTResutl = Result<NumValue, Error>;

impl NumValue {
    pub fn added_by(&self, num_value: &NumValue) -> RTResutl {
        match self {
            NumValue::BigInt { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::BigInt {
                            value: lvalue + value,
                            ctx,
                        })
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue.to_f64().unwrap() + value,
                            ctx,
                        })
                    }
                }
            }
            NumValue::Float { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue + value.to_f64().unwrap(),
                            ctx,
                        })
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue + value,
                            ctx,
                        })
                    }
                }
            }
        }
    }
    pub fn subed_by(&self, num_value: &NumValue) -> RTResutl {
        match self {
            NumValue::BigInt { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::BigInt {
                            value: lvalue - value,
                            ctx,
                        })
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue.to_f64().unwrap() - value,
                            ctx,
                        })
                    }
                }
            }
            NumValue::Float { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue - value.to_f64().unwrap(),
                            ctx,
                        })
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue - value,
                            ctx,
                        })
                    }
                }
            }
        }
    }
    pub fn divded_by(&self, num_value: &NumValue) -> RTResutl {
        match self {
            NumValue::BigInt { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        if value != &0.to_bigint().unwrap() {
                            Ok(NumValue::BigInt {
                                value: lvalue / value,
                                ctx,
                            })
                        } else {
                            Err(Error::DivdedByZero {
                                ctx,
                                detail: "Divided by zero".to_string(),
                            })
                        }
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        if value != &0f64 {
                            Ok(NumValue::Float {
                                value: lvalue.to_f64().unwrap() / value,
                                ctx,
                            })
                        } else {
                            Err(Error::DivdedByZero {
                                ctx,
                                detail: "Divided by zero".to_string(),
                            })
                        }
                    }
                }
            }
            NumValue::Float { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        if value != &0.to_bigint().unwrap() {
                            Ok(NumValue::Float {
                                value: lvalue / value.to_f64().unwrap(),
                                ctx,
                            })
                        } else {
                            Err(Error::DivdedByZero {
                                ctx,
                                detail: "Divided by zero".to_string(),
                            })
                        }
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        if value != &0f64 {
                            Ok(NumValue::Float {
                                value: lvalue / value,
                                ctx,
                            })
                        } else {
                            Err(Error::DivdedByZero {
                                ctx,
                                detail: "Divided by zero".to_string(),
                            })
                        }
                    }
                }
            }
        }
    }
    pub fn multed_by(&self, num_value: &NumValue) -> RTResutl {
        match self {
            NumValue::BigInt { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::BigInt {
                            value: lvalue * value,
                            ctx,
                        })
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue.to_f64().unwrap() * value,
                            ctx,
                        })
                    }
                }
            }
            NumValue::Float { value, ctx } => {
                let lvalue = value.clone();
                let lctx = &*ctx;
                match num_value {
                    NumValue::BigInt { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue * value.to_f64().unwrap(),
                            ctx,
                        })
                    }
                    NumValue::Float { value, ctx } => {
                        let ctx = lctx.merge(ctx);
                        Ok(NumValue::Float {
                            value: lvalue * value,
                            ctx,
                        })
                    }
                }
            }
        }
    }
    pub fn inverse(&self) -> RTResutl {
        todo!()
    }
}

impl Interpreter {
    pub fn evaluate(&mut self, node: Box<ASTNode>) -> RTResutl {
        match *node {
            ASTNode::Number { token, posctx } => self.visit_number(token, posctx),
            ASTNode::BinOp {
                token,
                posctx,
                left,
                right,
            } => self.visit_binop(token, posctx, left, right),
            ASTNode::Unary {
                token,
                posctx,
                node,
            } => self.visit_unary(token, posctx, node),
        }
    }

    pub fn visit_number(&self, token: Box<Token>, posctx: Box<PosCtx>) -> RTResutl {
        match *token {
            Token::Int { value } => Ok(NumValue::BigInt { value, ctx: posctx }),
            Token::Float { value } => Ok(NumValue::Float { value, ctx: posctx }),
            _ => Err(Error::TokenNotMatch {
                ctx: posctx,
                detail: String::from(format!("Got {:?}, Expect 'Int', 'Float'", *token)),
            }),
        }
    }
    pub fn visit_binop(
        &self,
        token: Box<Token>,
        posctx: Box<PosCtx>,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    ) -> RTResutl {
        let cur_posctx = posctx;
        let l = match *left {
            ASTNode::Number { token, posctx } => self.visit_number(token, posctx),
            ASTNode::BinOp {
                token,
                posctx,
                left,
                right,
            } => self.visit_binop(token, posctx, left, right),
            ASTNode::Unary {
                token,
                posctx,
                node,
            } => self.visit_unary(token, posctx, node),
        };
        let left_value: NumValue;
        if let Err(error) = l {
            return Err(error);
        } else {
            left_value = l.unwrap();
        }

        let r = match *right {
            ASTNode::Number { token, posctx } => self.visit_number(token, posctx),
            ASTNode::BinOp {
                token,
                posctx,
                left,
                right,
            } => self.visit_binop(token, posctx, left, right),
            ASTNode::Unary {
                token,
                posctx,
                node,
            } => self.visit_unary(token, posctx, node),
        };

        let right_value: NumValue;
        if let Err(error) = r {
            return Err(error);
        } else {
            right_value = r.unwrap();
        }

        match *token {
            Token::Plus { value: _ } => left_value.added_by(&right_value),
            Token::Minus { value: _ } => left_value.subed_by(&right_value),
            Token::Mul { value: _ } => left_value.multed_by(&right_value),
            Token::Div { value: _ } => left_value.divded_by(&right_value),
            _ => Err(Error::TokenNotMatch {
                ctx: cur_posctx,
                detail: String::from(format!("Got {:?}, Expect '+', '-', '*', '/'", *token)),
            }),
        }
    }
    pub fn visit_unary(
        &self,
        token: Box<Token>,
        posctx: Box<PosCtx>,
        node: Box<ASTNode>,
    ) -> RTResutl {
        let cur_posctx = posctx;
        let next = match *node {
            ASTNode::Number { token, posctx } => self.visit_number(token, posctx),
            ASTNode::BinOp {
                token,
                posctx,
                left,
                right,
            } => self.visit_binop(token, posctx, left, right),
            ASTNode::Unary {
                token,
                posctx,
                node,
            } => self.visit_unary(token, posctx, node),
        };
        let cur_value: NumValue;
        if let Err(error) = next {
            return Err(error);
        } else {
            cur_value = next.unwrap();
        }

        match *token {
            Token::Plus { value: _ } => Ok(cur_value),
            Token::Minus { value: _ } => cur_value.inverse(),
            _ => Err(Error::TokenNotMatch {
                ctx: cur_posctx,
                detail: String::from(format!("Got {:?}, Expect '+', '-'", *token)),
            }),
        }
    }
}

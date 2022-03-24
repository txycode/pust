use crate::position::PosCtx;
use crate::token::Token;

#[derive(Debug)]
pub enum ASTNode {
    Number {
        token: Box<Token>,
        posctx: Box<PosCtx>,
    },
    BinOp {
        token: Box<Token>,
        posctx: Box<PosCtx>,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Unary {
        token: Box<Token>,
        posctx: Box<PosCtx>,
        node: Box<ASTNode>,
    },
}

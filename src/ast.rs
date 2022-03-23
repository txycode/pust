use crate::token::Token;

#[derive(Debug)]
pub enum ASTNode {
    Number {
        token: Box<Token>,
    },
    BinOp {
        token: Box<Token>,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Unary {
        token: Box<Token>,
        node: Box<ASTNode>,
    },
}

use crate::position::PosCtx;

#[derive(Debug)]
pub enum Error {
    IllegalChar { ctx: Box<PosCtx>, detail: String },
    InvalidSyntax { ctx: Box<PosCtx>, detail: String },
    TokenNotMatch { ctx: Box<PosCtx>, detail: String },
    DivdedByZero { ctx: Box<PosCtx>, detail: String },
}

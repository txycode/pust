use crate::position::PosCtx;

#[derive(Debug)]
pub enum Error {
    IllegalChar { ctx: Box<PosCtx>, detail: String },
    InvalidSyntax { ctx: Box<PosCtx>, detail: String },
}

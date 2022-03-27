use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub enum Token {
    Int { value: BigInt },
    Float { value: f64 },
    Plus { value: char },
    Minus { value: char },
    Mul { value: char },
    Div { value: char },
    Lpar { value: char },
    Rpar { value: char },
    Assign { value: char },
    Pow { value: String },
    Ident { value: String },
    EndOfFile,
}

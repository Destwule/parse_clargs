#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(u64),
    Bool(bool)
}

#[derive(PartialEq, Clone)]
pub enum ValueKind {
    Str,
    Num,
    Bool,
    UNSPECIFIED
}

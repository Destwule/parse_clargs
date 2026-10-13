#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Str(String),
    Num(u64),
    Bool(bool)
}

#[derive(PartialEq, Clone, Debug)]
pub enum ValueKind {
    Str,
    Num,
    Bool,
    UNSPECIFIED
}

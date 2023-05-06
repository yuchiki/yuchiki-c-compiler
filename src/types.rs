#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    IntType,
    PointerType(Box<Type>),
}

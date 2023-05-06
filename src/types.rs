#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    IntType,
    PointerType(Box<Type>),
}

impl Type {
    pub const fn get_size(&self) -> usize {
        match self {
            Self::PointerType(_) => 8,
            Self::IntType => 4,
        }
    }
}

pub type FunctionType = (Vec<Type>, Box<Type>);

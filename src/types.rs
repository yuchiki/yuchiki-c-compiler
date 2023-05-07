#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    IntTyp,
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
}

impl Type {
    #[allow(dead_code)]
    pub fn get_size(&self) -> usize {
        match self {
            Self::Pointer(_) => 8,
            Self::IntTyp => 4,
            Self::Array(t, n) => t.get_size() * n,
        }
    }
}

pub type FunctionType = (Vec<Type>, Box<Type>);

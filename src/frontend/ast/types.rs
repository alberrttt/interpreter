use super::literal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub data_type: Primitive,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Primitive {
    Number,
    String,
    Boolean,
    #[default]
    Void,
}
impl From<Literal> for Primitive {
    fn from(value: Literal) -> Self {
        match value.0 {
            Literals::Number(_) => Primitive::Number,
            Literals::String(_) => Primitive::String,
            Literals::Bool(_) => Primitive::Boolean,
            Literals::Void => Primitive::Void,
        }
    }
}

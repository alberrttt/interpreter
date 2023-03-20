use super::{
    declaration::function::{FunctionDeclaration, Parameter},
    identifier::Identifier,
    literal::*,
};
pub trait Typecheck {
    fn typecheck(&self);
}
#[derive(Debug, PartialEq, Clone)]
pub enum Signature {
    Function {
        params: Vec<Primitive>,
        return_type: Box<Primitive>,
    },
    Variable(Box<Primitive>),
}

impl From<FunctionDeclaration> for Signature {
    fn from(value: FunctionDeclaration) -> Self {
        let params: Vec<Primitive> = value
            .parameters
            .into_iter()
            .map(|param| param.type_annotation.expect("inference is not added (yet)"))
            .collect();
        let return_type = Box::new(value.return_type.unwrap_or_else(|| {
            println!(
                "inference hasn't been added yet, so it will infer that it is a void return type."
            );
            Primitive::Void
        }));
        Signature::Function {
            params,
            return_type,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Primitive {
    Number,
    String,
    Boolean,
    #[default]
    Void,
}

impl From<Identifier> for Primitive {
    fn from(value: Identifier) -> Self {
        match value.value.lexeme.as_ref() {
            "number" => Primitive::Number,
            "string" => Primitive::String,
            "bool" | "boolean" => Primitive::Boolean,
            "void" => Primitive::Void,
            string => panic!("{string}"),
        }
    }
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

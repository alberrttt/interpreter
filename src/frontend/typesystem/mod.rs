use std::fmt::Display;

use crate::println_with_source;

use super::{
    compiler::Compiler,
    declaration::function::{FunctionDeclaration, Parameter},
    identifier::Identifier,
    literal::*,
};
pub trait Typecheck {
    fn typecheck(&self, against: &Signature) -> bool;
}
pub trait ResolveSignature {
    fn resolve_signature(&self, compiler: &mut Compiler) -> Signature;
}
#[derive(Debug, PartialEq, Clone)]
pub enum Signature {
    Function(FunctionSignature),
    Variable(Box<Signature>),
    Primitive(Primitive),
    Parameter(ParameterSignature),
}
impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signature::Function(function_signature) => {
                write!(f, "FunctionSignature({function_signature:?})",)
            }
            Signature::Variable(signature) => write!(f, "{signature}"),
            Signature::Primitive(primitive) => write!(f, "{primitive:?}",),
            Signature::Parameter(parameter_signature) => {
                write!(f, "ParameterSignature({parameter_signature:?})",)
            }
        }
    }
}
// create a parameter signature
#[derive(Debug, PartialEq, Clone)]
pub struct ParameterSignature {
    pub name: Identifier,
    pub type_annotation: Option<Box<Signature>>,
}
impl From<Parameter> for ParameterSignature {
    fn from(value: Parameter) -> Self {
        ParameterSignature {
            name: value.name,
            type_annotation: value.type_annotation.map(|type_annotation| {
                let type_annotation = Signature::from(Primitive::from(type_annotation));
                Box::new(type_annotation)
            }),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionSignature {
    pub params: Vec<ParameterSignature>,
    pub return_type: Box<Primitive>,
}
impl From<Primitive> for Signature {
    fn from(value: Primitive) -> Self {
        Signature::Primitive(value)
    }
}
impl From<FunctionDeclaration> for Signature {
    fn from(value: FunctionDeclaration) -> Self {
        let params: Vec<ParameterSignature> = value
            .parameters
            .into_iter()
            .map(|param| {
                println_with_source!("probably seperate this part out into it's own function");
                let type_annotation =
                    Primitive::from(param.type_annotation.expect("inference is not added (yet)"));
                return ParameterSignature {
                    name: param.name,
                    type_annotation: Some(Box::new(Signature::Primitive(type_annotation))),
                };
            })
            .collect();
        let return_type = Box::new(value.return_type.unwrap_or_else(|| {
            println!(
                "inference hasn't been added yet, so it will infer that it is a void return type."
            );
            Primitive::Void
        }));
        Signature::Function(FunctionSignature {
            params,
            return_type,
        })
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

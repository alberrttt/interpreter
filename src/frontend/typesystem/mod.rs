use std::{fmt::Display, primitive};

use strum::Display;

use crate::debug_println;

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
                write!(f, "{function_signature}",)
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
impl Display for FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "func({}) -> {}",
            self.params
                .iter()
                .map(|f| f.type_annotation.as_ref().unwrap().to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.return_type.as_ref()
        )
    }
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
                debug_println!("probably seperate this part out into it's own function");
                let type_annotation = Primitive::from(
                    param
                        .type_annotation
                        .expect("inference has not been added (yet)"),
                );
                ParameterSignature {
                    name: param.name,
                    type_annotation: Some(Box::new(Signature::Primitive(type_annotation))),
                }
            })
            .collect();
        let return_type = Box::new(value.return_type.unwrap_or_else(|| {
            debug_println!(
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
    Function,
}
impl From<Signature> for Primitive {
    fn from(signature: Signature) -> Self {
        match signature {
            Signature::Function(function_signature) => Primitive::Function,
            Signature::Variable(variable) => Primitive::from(*variable),
            Signature::Primitive(primitive) => primitive,
            Signature::Parameter(param) => (*param.type_annotation.unwrap()).into(),
        }
    }
}
impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}
impl From<Identifier> for Primitive {
    fn from(value: Identifier) -> Self {
        match value.value.lexeme.as_ref() {
            "number" => Primitive::Number,
            "string" => Primitive::String,
            "bool" | "boolean" => Primitive::Boolean,
            "void" => Primitive::Void,
            "function" => Primitive::Function,
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

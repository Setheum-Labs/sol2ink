// MIT License

// Copyright (c) 2022 Supercolony

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use convert_case::{
    Case::Snake,
    Casing,
};
use solang_parser::pt::{
    Expression as SolangExpression,
    Identifier,
    Statement as SolangStatement,
};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum ArrayType {
    DynamicArray,
    FixedSizeArray,
    Mapping,
}

#[derive(Clone, Default)]
pub struct Contract {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub constructor: Function,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub imports: HashSet<String>,
    pub contract_doc: Vec<String>,
    pub modifiers: Vec<Modifier>,
}

pub struct Library {
    pub name: String,
    pub fields: Vec<ContractField>,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    pub imports: HashSet<String>,
    pub libraray_doc: Vec<String>,
}

pub struct Interface {
    pub name: String,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub function_headers: Vec<FunctionHeader>,
    pub imports: HashSet<String>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct ContractField {
    pub field_type: Type,
    pub name: String,
    pub comments: Vec<String>,
    pub initial_value: Option<Expression>,
    pub constant: bool,
    pub public: bool,
}

#[derive(Clone)]
pub struct Modifier {
    pub header: FunctionHeader,
    pub statements: Vec<Statement>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct Event {
    pub name: String,
    pub fields: Vec<EventField>,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct EventField {
    pub indexed: bool,
    pub field_type: Type,
    pub name: String,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumField>,
    pub comments: Vec<String>,
}

#[derive(Default, Clone)]
pub struct EnumField {
    pub name: String,
    pub comments: Vec<String>,
}

#[derive(Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub comments: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
    pub comments: Vec<String>,
}

#[derive(Default, Clone)]
pub struct Function {
    pub header: FunctionHeader,
    pub body: WrappedStatement,
}

#[derive(Default, Clone)]
pub struct FunctionHeader {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub external: bool,
    pub view: bool,
    pub payable: bool,
    pub return_params: Vec<FunctionParam>,
    pub comments: Vec<String>,
    pub modifiers: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub param_type: Type,
}

#[derive(Default, Clone)]
pub struct WrappedStatement(pub Option<SolangStatement>);

#[derive(Default, Clone)]
pub struct WrappedExpression(pub Option<SolangExpression>);

impl WrappedExpression {
    pub fn wrap(expression: &SolangExpression) -> Self {
        Self(Some(expression.clone()))
    }
}

#[derive(Default, Clone)]
pub struct WrappedIdentifier(pub Option<Identifier>);

impl WrappedIdentifier {
    pub fn wrap(identifier: &Identifier) -> Self {
        Self(Some(identifier.clone()))
    }

    pub fn wrap_option(identifier: &Option<Identifier>) -> Self {
        Self(identifier.clone())
    }

    pub fn parse(&self) -> String {
        match &self.0 {
            Some(identifier) => identifier.name.clone(),
            None => String::from("_"),
        }
    }

    pub fn parse_snake(&self) -> String {
        match &self.0 {
            Some(identifier) => identifier.name.to_case(Snake),
            None => String::from("_"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Assign(Expression, Expression, Operation),
    ArrayFunctionCall(Expression, String, Expression),
    Break,
    Catch(Vec<Statement>),
    Comment(String),
    Declaration(String, String, Option<Expression>),
    Delete(Box<Expression>, Vec<Expression>),
    Loop(
        Option<Box<Statement>>,
        Expression,
        Option<Box<Statement>>,
        Vec<Statement>,
    ),
    Else(Vec<Statement>),
    ElseIf(Condition, Vec<Statement>),
    Emit(String, Vec<Expression>),
    FunctionCall(Expression),
    Group(Vec<Statement>),
    If(Condition, Vec<Statement>),
    ModifierBody,
    Raw(String),
    Require(Condition, Expression, bool),
    Return(Expression),
    Ternary(Condition, Box<Statement>, Box<Statement>),
    Try(Vec<Statement>),
    While(
        Option<Box<Statement>>,
        Expression,
        Option<Box<Statement>>,
        Vec<Statement>,
    ),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub left: Expression,
    pub operation: Operation,
    pub right: Option<Expression>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Add,
    AddAssign,
    AddOne,
    AndAssign,
    Assign,
    BitwiseAnd,
    BitwiseOr,
    Div,
    DivAssign,
    Equal,
    GreaterThanEqual,
    GreaterThan,
    LessThanEqual,
    LessThan,
    LogicalAnd,
    LogicalOr,
    Modulo,
    Mul,
    MulAssign,
    Not,
    NotEqual,
    OrAssign,
    Pow,
    Subtract,
    SubtractOne,
    SubtractAssign,
    ShiftLeft,
    ShiftRight,
    True,
    Xor,
}

impl Operation {
    pub fn negate(&self) -> Operation {
        match self {
            Operation::BitwiseAnd => Operation::BitwiseOr,
            Operation::BitwiseOr => Operation::BitwiseAnd,
            Operation::Equal => Operation::NotEqual,
            Operation::GreaterThanEqual => Operation::LessThan,
            Operation::GreaterThan => Operation::LessThanEqual,
            Operation::LessThanEqual => Operation::GreaterThan,
            Operation::LessThan => Operation::GreaterThanEqual,
            // TODO a and b = neg(a) or neg (b)
            Operation::LogicalAnd => Operation::LogicalOr,
            Operation::LogicalOr => Operation::LogicalAnd,
            Operation::Not => Operation::True,
            Operation::NotEqual => Operation::Equal,
            _ => Operation::Not,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    AccountId(Option<String>),
    Arithmetic(Box<Expression>, Box<Expression>, Operation),
    BlockTimestamp(Option<String>),
    DynamicArray(Box<Expression>, Vec<Expression>),
    FixedSizeArray(Box<Expression>, Vec<Expression>),
    Cast(bool, String, Box<Expression>),
    ComplexMapping(Vec<Expression>),
    Condition(Box<Condition>),
    Constant(String),
    Enclosed(Box<Expression>),
    EnvCaller(Option<String>),
    FunctionCall(String, Vec<Expression>, Option<String>, bool, bool),
    IsZero(Box<Expression>),
    Literal(String),
    Logical(Box<Expression>, Operation, Box<Expression>),
    Member(String, Option<String>),
    Mapping(Box<Expression>, Vec<Expression>, Option<Box<Expression>>),
    Modifier(String),
    NewArray(String, Box<Expression>),
    StructArg(String, Box<Expression>),
    StructInit(String, Vec<Expression>),
    Ternary(Box<Condition>, Box<Expression>, Box<Expression>),
    TransferredValue(Option<String>),
    WithSelector(Box<Expression>, Box<Expression>),
    ZeroAddressInto,
}

pub enum Block {
    Assembly,
    Catch,
    Else,
    ElseIf,
    If,
    Try,
    Unchecked,
    While,
}

#[derive(Clone, Debug)]
pub enum Type {
    AccountId,
    Bool,
    String,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    DynamicBytes,
    Variable(String),
    Mapping(Vec<Type>, Box<Type>),
    None,
}

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
    pub body: Option<Statement>,
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

#[derive(Clone, Debug)]
pub enum Statement {
    Assembly(Vec<String>),
    Block(Vec<Statement>),
    Break,
    Continue,
    DoWhile(Box<Statement>, Expression),
    Emit(Expression),
    Error,
    Expression(Expression),
    For(
        Option<Box<Statement>>,
        Option<Expression>,
        Option<Box<Statement>>,
        Option<Box<Statement>>,
    ),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Return(Option<Expression>),
    Revert(String, Vec<Expression>),
    RevertNamedArgs,
    Try(Expression),
    UncheckedBlock(Vec<Statement>),
    VariableDefinition(Expression, Option<Expression>),
    While(Expression, Box<Statement>),
}

#[derive(Clone, Debug)]
pub enum Expression {
    ArraySubscript(Box<Expression>, Option<Box<Expression>>),
    Assign(Box<Expression>, Box<Expression>),
    AssignAdd(Box<Expression>, Box<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    MappingSubscript(Box<Expression>, Vec<Expression>),
    MemberAccess(Box<Expression>, String),
    MoreEqual(Box<Expression>, Box<Expression>),
    New(Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    NumberLiteral(String),
    Or(Box<Expression>, Box<Expression>),
    PostDecrement(Box<Expression>),
    PostIncrement(Box<Expression>),
    PreDecrement(Box<Expression>),
    PreIncrement(Box<Expression>),
    StringLiteral(Vec<String>),
    Subtract(Box<Expression>, Box<Expression>),
    Type(Box<Type>),
    Variable(String, bool),
    VariableDeclaration(Box<Type>, String),
}

#[derive(Clone, Debug)]
pub enum Type {
    AccountId,
    Array(Box<Type>, Option<Expression>),
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

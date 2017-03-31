// Our internal representation of the language

use std::collections::HashMap;

pub enum Token {
  Colon, Semicolon, Comma,
  OpenParen, CloseParen, OpenBracket, CloseBracket, OpenBrace, CloseBrace,
  ID(String), Integer(i64), Float(f64), String(String),
  True, False, Nil, EOF
}

pub struct Block {
  pub expressions: Vec<Expression>
}

pub enum Expression {
  Nil, True, False, Integer(i64), Float(f64), String(String), List(List),
  Call(Call), Definition(Definition)
  //, Hash(Hash)
}

pub struct List {
  pub items: Vec<Expression>
}

pub struct Call {
  pub id: String,
  pub params: Vec<Expression>
}

pub struct Definition {
  pub id: String,
  pub params: Vec<String>,
  pub block: Block
}

pub struct Scope {
  pub bindings: HashMap<String, FunctionOrValue>
}

pub enum FunctionOrValue {
  Function(Function), Value(Evaluation)
}

pub enum Evaluation {
  Nil, True, False, Integer(i64), Float(f64), String(String), List(ListEval),
  Function(Function), Exception(Exception)
  //, Hash(Hash)
}

pub struct ListEval {
  pub items: Vec<Evaluation>
}

pub struct Function {
  pub params: Vec<String>,
  pub block: Block
}

pub struct Exception {
  pub flavor: ExceptionType,
  pub payload: Box<Evaluation>,
  pub stack: Vec<String>
}

pub enum ExceptionType {
  Return, Error, ArityError, ParseError, TypeError, TypeMismatch, DivByZero,
  RuntimeError, UndefError, RedefError
}

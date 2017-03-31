use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

use encoding::Token;

use encoding::Expression;

use encoding::Scope;
use encoding::FunctionOrValue;
use encoding::Evaluation;
use encoding::Function;
use encoding::Exception;
use encoding::ExceptionType;

impl Debug for Token {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Token::Colon => "COLON".to_string(),
      &Token::Semicolon => "SEMICOLON".to_string(),
      &Token::Comma => "COMMA".to_string(),
      &Token::True => "TRUE".to_string(),
      &Token::False => "FALSE".to_string(),
      &Token::Nil => "NIL".to_string(),
      &Token::OpenParen => "OPENPAREN".to_string(),
      &Token::CloseParen => "CLOSEPAREN".to_string(),
      &Token::OpenBracket => "OPENBRACKET".to_string(),
      &Token::CloseBracket => "CLOSEBRACKET".to_string(),
      &Token::OpenBrace => "OPENBRACE".to_string(),
      &Token::CloseBrace => "CLOSEBRACE".to_string(),
      &Token::ID(ref x) => "ID:".to_string() + &x,
      &Token::Integer(ref x) => "INTEGER:".to_string() + &x.to_string(),
      &Token::Float(ref x) => "FLOAT:".to_string() + &x.to_string(),
      &Token::String(ref x) => "STRING:".to_string() + &x,
      &Token::EOF => "EOF".to_string(),
    };
    write!(f, "{}", s)
  }
}

impl Debug for Expression {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Expression::Nil => "NIL".to_string(),
      &Expression::True => "TRUE".to_string(),
      &Expression::False => "FALSE".to_string(),
      &Expression::Integer(ref x) => "INTEGER:".to_string() + &x.to_string(),
      &Expression::Float(ref x) => "FLOAT:".to_string() + &x.to_string(),
      &Expression::String(ref x) => "STRING:".to_string() + &x,
      &Expression::List(ref x) => {
        let mut s2 = "LIST:[ ".to_string();
        for i in &x.items {
          s2 += &format!("{:?} ", i);
        }
        s2 += "]";
        s2
      },
      &Expression::Call(ref x) => {
        let mut s2 = "CALL:".to_string() + &x.id;
        if x.params.len() > 0 {
          s2 += "( ";
          for i in &x.params {
            s2 += &format!("{:?} ", i);
          }
          s2 += ")";
        }
        s2
      },
      &Expression::Definition(ref x) => {
        let mut s2 = "DEFINITION:".to_string() + &x.id;
        if x.params.len() > 0 {
          s2 += "( ";
          for i in &x.params {
            s2 += &i;
            s2 += &" ".to_string();
          }
          s2 += "):";
        }
        for i in &x.block.expressions {
          s2 += &format!(" {:?};", i);
        }
        s2
      },
    };
    write!(f, "{}", s)
  }
}

impl Debug for Scope {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = "SCOPE:".to_string();
    for (id, f) in &self.bindings {
      s += &format!(" {}:{:?}", id, f);
    }
    write!(f, "{}", s)
  }
}

impl Debug for FunctionOrValue {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &FunctionOrValue::Function(ref func) => {
        format!("{:?}", func)
      },
      &FunctionOrValue::Value(ref value) => {
        format!("{:?}", value)
      },
    };
    write!(f, "{}", s)
  }
}

impl Debug for Evaluation {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Evaluation::Nil => "NIL".to_string(),
      &Evaluation::True => "TRUE".to_string(),
      &Evaluation::False => "FALSE".to_string(),
      &Evaluation::Integer(ref x) => "INTEGER:".to_string() + &x.to_string(),
      &Evaluation::Float(ref x) => "FLOAT:".to_string() + &x.to_string(),
      &Evaluation::String(ref x) => "STRING:".to_string() + &x,
      &Evaluation::List(ref x) => {
        let mut s2 = "LIST:[ ".to_string();
        for i in &x.items {
          s2 += &format!("{:?} ", i);
        }
        s2 += "]";
        s2
      },
      &Evaluation::Exception(ref x) => {
        let mut s2 = format!("EXCEPTION:[{}, ", x.flavor);
        s2 += &format!("{}, ", x.payload);
        let mut stack = Vec::new();
        for i in &x.stack {
          stack.push(i.clone());
        }
        s2 += &stack.join(", ");
        s2 += "]]";
        s2
      },
      &Evaluation::Function(ref x) => {
        let mut s2 = "FUNCTION:".to_string();
        if x.params.len() > 0 {
          s2 += "( ";
          for i in &x.params {
            s2 += &i;
            s2 += &" ".to_string();
          }
          s2 += "):";
        }
        for i in &x.block.expressions {
          s2 += &format!(" {:?};", i);
        }
        s2
      },
    };
    write!(f, "{}", s)
  }
}

impl Debug for Function {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = "( ".to_string();
    for p in &self.params {
      s += &p;
      s += " ";
    }
    s += "):";
    if self.block.expressions.len() == 1 {
      s += &format!("{:?}", &self.block.expressions[0]);
    } else {
      s += "<...>";
    }
    write!(f, "{}", s)
  }
}

impl Display for Evaluation {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &Evaluation::Nil => "nil".to_string(),
      &Evaluation::True => "true".to_string(),
      &Evaluation::False => "false".to_string(),
      &Evaluation::Integer(x) => x.to_string(),
      &Evaluation::Float(x) => x.to_string(),
      &Evaluation::String(ref x) => format!("\"{}\"", x),
      &Evaluation::List(ref x) => {
        let mut s2 = "[".to_string();
        let mut items = Vec::new();
        for i in &x.items {
          items.push(format!("{}", i));
        }
        s2 += &items.join(", ");
        s2 += "]";
        s2
      },
      &Evaluation::Exception(ref x) => {
        let mut s2 = format!("[{}, ", x.flavor);
        s2 += &format!("{}, ", x.payload);
        let mut stack = Vec::new();
        for i in &x.stack {
          stack.push(i.clone());
        }
        s2 += &stack.join(", ");
        s2 += "]]";
        s2
      },
      &Evaluation::Function(ref x) => {
        let mut s2 = "".to_string();
        if x.params.len() > 0 {
          s2 += "(";
          let mut params = Vec::new();
          for p in &x.params {
            params.push(p.clone());
          }
          s2 += &params.join(", ");
          s2 += ")";
        }
        s2 += ":<...>";
        s2
      },
    };
    write!(f, "{}", s)
  }
}

impl Display for Exception {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let mut s = format!("\nRUNTIME EXCEPTION: {}\n{}:\n\n  calling context:\n",
                        self.flavor.to_string().to_uppercase(), self.payload);
    let mut n = self.stack.len();
    for i in &self.stack {
      s += &format!("   -- called from function {}: {}\n", n - 1, i);
      n -= 1;
    }
    write!(f, "{}", s)
  }
}

impl Display for ExceptionType {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let s = match self {
      &ExceptionType::Return => "return".to_string(),
      &ExceptionType::Error => "error".to_string(),
      &ExceptionType::ArityError => "arity error".to_string(),
      &ExceptionType::ParseError => "parse error".to_string(),
      &ExceptionType::TypeError => "type error".to_string(),
      &ExceptionType::TypeMismatch => "type mismatch".to_string(),
      &ExceptionType::DivByZero => "division by zero".to_string(),
      &ExceptionType::RuntimeError => "runtime error".to_string(),
      &ExceptionType::UndefError => "undefined function".to_string(),
      &ExceptionType::RedefError => "redefinition error".to_string(),
    };
    write!(f, "{}", s)
  }
}

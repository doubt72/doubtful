use std::collections::HashMap;

use primitives;

use encoding::Block;
use encoding::Expression;
use encoding::List;
use encoding::Call;
use encoding::Definition;

use encoding::Scope;
use encoding::FunctionOrValue;
use encoding::Evaluation;
use encoding::ListEval;
use encoding::Function;

impl List {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    let mut list = ListEval { items: Vec::new() };
    for i in &self.items {
      list.items.push(i.evaluate(scope));
    }
    Evaluation::List(list)
  }

  pub fn clone(&self) -> List {
    let mut list = List { items: Vec::new() };
    for i in &self.items {
      list.items.push(i.clone());
    }
    list
  }
}

impl Call {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    let mut check = false;
    let rc;

    // Debug here:
    //println!("CALLING {} IN", &self.id);
    //for n in 0..scope.len() {
    //  println!("  {:?}", scope[n]);
    //}

    // Search through scopes in reverse order for function (or value)
    let mut binding = FunctionOrValue::Value(Evaluation::Nil);
    for x in (0..scope.len()).rev() {
      if scope[x].bindings.contains_key(&self.id) {
        check = true;
        binding = scope[x].bindings[&self.id].clone();
        break;
      }
    }
    if check {
      match binding {
        FunctionOrValue::Function(ref func) => {
          // Add a scope for the function parameters, evaluate, and populate
          let mut p_scope = Scope { bindings: HashMap::new() };
          if self.params.len() != func.params.len() {
            // TODO: better error handling
            panic!(format!("arity mismatch in call to {} ({}, expected {})",
                           self.id, self.params.len(), func.params.len()));
          }
          for y in 0..self.params.len() {
            let mut block = Block { expressions: Vec::new() };
            block.expressions.push(self.params[y].clone());
            p_scope.bindings.insert(func.params[y].clone(),
                                    FunctionOrValue::Value(block.evaluate(scope)));
          }
          scope.push(p_scope);
          rc = func.block.evaluate(scope);
          scope.pop();
        },
        FunctionOrValue::Value(ref value) => {
          // This value has already been evaluated
          rc = value.clone();
        },
      }
    } else {
      // Try low-level system functions
      let mut params = Vec::new();
      for p in &self.params {
        params.push(p.evaluate(scope));
      }
      rc = primitives::system_functions(self.id.clone(), params);
    }
    rc
  }

  pub fn clone(&self) -> Call {
    let mut call = Call { id: self.id.clone(), params: Vec::new() };
    for p in &self.params {
      call.params.push(p.clone());
    }
    call
  }
}

impl Definition {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    let last = scope.last_mut();
    if let Some(s) = last {
      if s.bindings.contains_key(&self.id) {
        // TODO: do better than panic
        panic!(format!("attempt to redefine {}", self.id));
      }
      let mut func = Function { params: Vec::new(), block: self.block.clone() };
      for p in &self.params {
        func.params.push(p.clone());
      }
      s.bindings.insert(self.id.clone(),
                         FunctionOrValue::Function(func.clone()));
      Evaluation::Function(func)
    } else {
      panic!("internal error: no scope supplied to definition evaluation");
    }
  }

  pub fn clone(&self) -> Definition {
    let mut def = Definition { id: self.id.clone(), params: Vec::new(),
                               block: self.block.clone() };
    for p in &self.params {
      def.params.push(p.clone());
    }
    def
  }
}

impl Expression {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {
    match self {
      &Expression::Nil => {
        Evaluation::Nil
      },
      &Expression::True => {
        Evaluation::True
      },
      &Expression::False => {
        Evaluation::False
      },
      &Expression::Integer(x) => {
        Evaluation::Integer(x)
      },
      &Expression::Float(x) => {
        Evaluation::Float(x)
      },
      &Expression::String(ref s) => {
        Evaluation::String(s.clone())
      },
      &Expression::List(ref list) => {
        list.evaluate(scope)
      },
      &Expression::Call(ref call) => {
        call.evaluate(scope)
      },
      &Expression::Definition(ref def) => {
        def.evaluate(scope)
      },
    }
  }

  pub fn clone(&self) -> Expression {
    match self {
      &Expression::Nil => {
        Expression::Nil
      },
      &Expression::True => {
        Expression::True
      },
      &Expression::False => {
        Expression::False
      },
      &Expression::Integer(x) => {
        Expression::Integer(x)
      },
      &Expression::Float(x) => {
        Expression::Float(x)
      },
      &Expression::String(ref s) => {
        Expression::String(s.clone())
      },
      &Expression::List(ref list) => {
        Expression::List(list.clone())
      },
      &Expression::Call(ref call) => {
        Expression::Call(call.clone())
      },
      &Expression::Definition(ref def) => {
        Expression::Definition(def.clone())
      },
    }
  }
}

impl Block {
  pub fn evaluate(&self, scope: &mut Vec<Scope>) -> Evaluation {

    // Add current context
    let current = Scope { bindings: HashMap::new() };
    scope.push(current);

    // Evaluate
    let mut value = Evaluation::Nil;
    for e in &self.expressions {
      value = e.evaluate(scope);
      // Debug output:
      //println!("{:?}", value);
    }
    // Current context going out of scope
    scope.pop();
    value
  }

  pub fn clone(&self) -> Block {
    let mut rc = Block { expressions: Vec::new() };
    for i in &self.expressions {
      rc.expressions.push(i.clone());
    }
    rc
  }
}

impl FunctionOrValue {
  pub fn clone(&self) -> FunctionOrValue {
    match self {
      &FunctionOrValue::Function(ref func) => {
        FunctionOrValue::Function(func.clone())
      },
      &FunctionOrValue::Value(ref value) => {
        FunctionOrValue::Value(value.clone())
      },
    }
  }
}

impl Evaluation {
  pub fn clone(&self) -> Evaluation {
    match self {
      &Evaluation::Nil => {
        Evaluation::Nil
      },
      &Evaluation::True => {
        Evaluation::True
      },
      &Evaluation::False => {
        Evaluation::False
      },
      &Evaluation::Integer(x) => {
        Evaluation::Integer(x)
      },
      &Evaluation::Float(x) => {
        Evaluation::Float(x)
      },
      &Evaluation::String(ref s) => {
        Evaluation::String(s.clone())
      },
      &Evaluation::List(ref list) => {
        Evaluation::List(list.clone())
      },
      &Evaluation::Function(ref func) => {
        Evaluation::Function(func.clone())
      },
    }
  }
}

impl ListEval {
  pub fn clone(&self) -> ListEval {
    let mut list = ListEval { items: Vec::new() };
    for i in &self.items {
      list.items.push(i.clone());
    }
    list
  }
}

impl Function {
  pub fn clone(&self) -> Function {
    let mut func = Function { params: Vec::new(), block: self.block.clone() };
    for p in &self.params {
      func.params.push(p.clone());
    }
    func
  }
}

// Primitive functions

use encoding::Evaluation;
use encoding::ListEval;

fn expect_args(count: usize, params: &Vec<Evaluation>, id: String) {
  if count != params.len() {
    panic!(format!("{} : expected {} arguments but got {}",
                   id, count, params.len()));
  }
}

// TODO: replace all those panics with proper runtime errors
pub fn system_functions(id: String, params: Vec<Evaluation>) -> Evaluation {
  match &*id {
    // Type Conversion
    "int" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::Float(x) => Evaluation::Integer(x as i64),
        Evaluation::String(ref s) => {
          match s.parse::<i64>() {
            Ok(n) => Evaluation::Integer(n),
            _ => panic!("int : unable to parse string"),
          }
        },
        _ => panic!("int : must pass float or string"),
      }
    },
    "float" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::Integer(x) => Evaluation::Float(x as f64),
        Evaluation::String(ref s) => {
          match s.parse::<f64>() {
            Ok(n) => Evaluation::Float(n),
            _ => panic!("float : unable to parse string"),
          }
        },
        _ => panic!("float : must pass int or string"),
      }
    },
    "string" => {
      expect_args(1, &params, id);
      Evaluation::String(format!("{}", params[0]))
    },
    // IO
    ">>" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::String(ref s) => {
          println!("{}", s);
          Evaluation::Nil
        },
        _ => {
          panic!(">> : string expected")
        },
      }
    },
    // MATH (plus appending things)
    "+" => {
      expect_args(2, &params, id);
      match params[0] {
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Integer(x + y),
            Evaluation::Float(y) => Evaluation::Float(x as f64 + y),
            _ => panic!("+ : mismatched types"),
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Float(x + y as f64),
            Evaluation::Float(y) => Evaluation::Float(x + y),
            _ => panic!("+ : mismatched types"),
          }
        },
        Evaluation::String(ref s) => {
          match params[1] {
            Evaluation::String(ref t) => {
              let mut rc = s.clone();
              rc += &t.clone();
              Evaluation::String(rc)
            },
            _ => panic!("+ : mismatched types"),
          }
        },
        Evaluation::List(ref list) => {
          let mut rc = ListEval { items: Vec::new() };
          for i in &list.items {
            rc.items.push(i.clone());
          }
          rc.items.push(params[1].clone());
          Evaluation::List(rc)
        },
        _ => panic!("+ : must add numbers, strings, or lists"),
      }
    },
    "-" => {
      expect_args(2, &params, id);
      match params[0] {
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Integer(x - y),
            Evaluation::Float(y) => Evaluation::Float(x as f64 - y),
            _ => panic!("- : must subtract numbers"),
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Float(x - y as f64),
            Evaluation::Float(y) => Evaluation::Float(x - y),
            _ => panic!("- : must subtract numbers"),
          }
        },
        _ => panic!("- : must subtract numbers"),
      }
    },
    "*" => {
      expect_args(2, &params, id);
      match params[0] {
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Integer(x * y),
            Evaluation::Float(y) => Evaluation::Float(x as f64 * y),
            _ => panic!("* : must multiply numbers"),
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Float(x * y as f64),
            Evaluation::Float(y) => Evaluation::Float(x * y),
            _ => panic!("* : must multiply numbers"),
          }
        },
        _ => panic!("* : must multiply numbers"),
      }
    },
    "/" => {
      expect_args(2, &params, id);
      match params[0] {
        // TODO: handle division by zero with proper error
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Integer(x / y),
            Evaluation::Float(y) => Evaluation::Float(x as f64 / y),
            _ => panic!("/ : must divide numbers"),
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Float(x / y as f64),
            Evaluation::Float(y) => Evaluation::Float(x / y),
            _ => panic!("/ : must divide numbers"),
          }
        },
        _ => panic!("/ : must divide numbers"),
      }
    },
    "%" => {
      expect_args(2, &params, id);
      match params[0] {
        // TODO: handle division by zero with proper error
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => Evaluation::Integer(x % y),
            _ => panic!("% : must use integers for modulus"),
          }
        },
        _ => panic!("% : must use integers for modulus"),
      }
    },
    // BOOLEAN
    "!" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::True => Evaluation::False,
        Evaluation::False => Evaluation::True,
        _ => panic!("! : argument must be 'true' or 'false'"),
      }
    }
    "&" => {
      expect_args(2, &params, id);
      match params[0] {
        // TODO: handle division by zero with proper error
        Evaluation::True => {
          match params[1] {
            Evaluation::True => Evaluation::True,
            Evaluation::False => Evaluation::False,
            _ => panic!("& : must use booleans for and"),
          }
        },
        Evaluation::False => {
          match params[1] {
            Evaluation::True => Evaluation::False,
            Evaluation::False => Evaluation::False,
            _ => panic!("& : must use booleans for and"),
          }
        },
        _ => panic!("& : must use booleans for and"),
      }
    },
    "|" => {
      expect_args(2, &params, id);
      match params[0] {
        // TODO: handle division by zero with proper error
        Evaluation::True => {
          match params[1] {
            Evaluation::True => Evaluation::True,
            Evaluation::False => Evaluation::True,
            _ => panic!("& : must use booleans for and"),
          }
        },
        Evaluation::False => {
          match params[1] {
            Evaluation::True => Evaluation::True,
            Evaluation::False => Evaluation::False,
            _ => panic!("& : must use booleans for and"),
          }
        },
        _ => panic!("& : must use booleans for and"),
      }
    },
    // CONTROL
    "?" => {
      expect_args(3, &params, id);
      match params[0] {
        Evaluation::True => params[1].clone(),
        Evaluation::False => params[2].clone(),
        _ => {
          panic!("? : first argument must be 'true' or 'false'");
        },
      }
    },
    "=" => {
      expect_args(2, &params, id);
      match params[0] {
        Evaluation::Nil => {
          match params[1] {
            Evaluation::Nil => Evaluation::True,
            _ => Evaluation::False,
          }
        },
        Evaluation::True => {
          match params[1] {
            Evaluation::True => Evaluation::True,
            _ => Evaluation::False,
          }
        },
        Evaluation::False => {
          match params[1] {
            Evaluation::False => Evaluation::True,
            _ => Evaluation::False,
          }
        },
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) =>
            {
              if x == y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => Evaluation::False,
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Float(y) =>
            {
              if x == y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => Evaluation::False,
          }
        },
        Evaluation::String(ref x) => {
          match params[1] {
            Evaluation::String(ref y) =>
            {
              if &x == &y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => Evaluation::False,
          }
        },
        Evaluation::List(ref x) => {
          match params[1] {
            Evaluation::List(ref y) => {
              if x.items.len() != y.items.len() {
                Evaluation::False
              } else {
                let mut rc = Evaluation::True;
                for n in 0..x.items.len() {
                  let cmp = vec![x.items[n].clone(), y.items[n].clone()];
                  match system_functions("=".to_string(), cmp) {
                    Evaluation::True => {
                      // do nothing, everything ok
                    },
                    _ => {
                      rc = Evaluation::False;
                      break;
                    }
                  }
                }
                rc
              }
            },
            _ => Evaluation::False,
          }
        },
        _ => Evaluation::False,
      }
    },
    ">" => {
      expect_args(2, &params, id);
      match params[0] {
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => {
              if x > y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            Evaluation::Float(y) => {
              if x as f64 > y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => panic!("> : must compare numbers"),
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Integer(y) => {
              if x > y as f64 {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            Evaluation::Float(y) => {
              if x > y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => panic!("> : must compare numbers"),
          }
        },
        _ => panic!("> : must compare numbers"),
      }
    },
    "<" => {
      expect_args(2, &params, id);
      match params[0] {
        Evaluation::Integer(x) => {
          match params[1] {
            Evaluation::Integer(y) => {
              if x < y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            Evaluation::Float(y) => {
              // weird parser bug seems to require parens here
              if (x as f64) < y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => panic!("< : must compare numbers"),
          }
        },
        Evaluation::Float(x) => {
          match params[1] {
            Evaluation::Integer(y) => {
              if x < y as f64 {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            Evaluation::Float(y) => {
              if x < y {
                Evaluation::True
              } else {
                Evaluation::False
              }
            },
            _ => panic!("< : must compare numbers"),
          }
        },
        _ => panic!("< : must compare numbers"),
      }
    },
    "substr" => {
      expect_args(3, &params, id);
      match params[0] {
        Evaluation::String(ref s) => {
          match params[1] {
            Evaluation::Integer(start) => {
              match params[2] {
                Evaluation::Integer(len) => {
                  let chars = s.chars();
                  if start as usize >= s.len() {
                    Evaluation::String("".to_string())
                  } else if (start + len) as usize >= s.len() {
                    let rc = chars.skip(start as usize).take(s.len() - start as usize).collect();
                    Evaluation::String(rc)
                  } else {
                    let rc = chars.skip(start as usize).take(len as usize).collect();
                    Evaluation::String(rc)
                  }
                },
                _ => panic!("substr : length must be integer"),
              }
            },
            _ => panic!("substr : starting index must be integer"),
          }
        },
        _ => panic!("substr : first argument must be string"),
      }
    },
    "strlen" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::String(ref s) => {
          Evaluation::Integer(s.chars().count() as i64)
        },
        _ => panic!("strlen : argument must be string"),
      }
    },
    "car" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::List(ref list) => {
          match list.items.first() {
            Some(item) => {
              item.clone()
            },
            _ => {
              panic!("car : attempt to get first item of empty list");
            }
          }
        },
        _ => panic!("car : argument must be list"),
      }
    },
    "cdr" => {
      expect_args(1, &params, id);
      match params[0] {
        Evaluation::List(ref list) => {
          let mut rc = list.clone();
          rc.items.remove(0);
          if rc.items.len() > 0 {
            Evaluation::List(rc)
          } else {
            Evaluation::Nil
          }
        },
        _ => panic!("cdr : argument must be list"),
      }
    },
    _ => panic!(format!("attempt to call undefined function: {}", id)),
  }
}

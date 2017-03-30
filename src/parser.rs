// Simple parser, which turns tokens into our internal encoding:

use encoding::Token;

use encoding::Block;
use encoding::Expression;
use encoding::List;
use encoding::Call;
use encoding::Definition;

fn get_token(tokens: &Vec<Token>, start: usize) -> &Token {
  if start > tokens.len() - 1 {
    // TODO: replace all panics with better handling
    panic!("unexpected end of file; statement unterminated")
  }
  &tokens[start]
}

fn parse_params(tokens: &Vec<Token>, start: usize) ->
  (Option<Vec<String>>, usize) {
  let mut rc = Vec::new();
  let mut index = start;
  loop {
    match get_token(tokens, index) {
      &Token::CloseParen => {
        index += 1;
        break;
      },
      &Token::ID(ref s) => {
        rc.push(s.clone());
        index += 1;
        match get_token(tokens, index) {
          &Token::Comma => {
            index += 1;
          },
          &Token::CloseParen => {
            // do nothing, next loop will catch it
          },
          _ => {
            return (None, 0);
          }
        }
      },
      _ => {
        return (None, 0);
      },
    }
  }
  (Some(rc), index)
}

fn parse_definition(tokens: &Vec<Token>, start: usize) ->
  (Option<Definition>, usize) {
  match get_token(tokens, start) {
    &Token::Colon => {
      // anonymous function with no parameters
      let (block, index) = parse_block(tokens, start + 1);
      (Some(Definition { id: "".to_string(), params: Vec::new(),
                         block: block }), index)
    },
    &Token::OpenParen => {
      // anonymous function
      let (opt, index) = parse_params(tokens, start + 1);
      match opt {
        Some(params) => {
          match get_token(tokens, index) {
            &Token::Colon => {
              let (block, last) = parse_block(tokens, index + 1);
              (Some(Definition { id:"".to_string(), params: params,
                                 block: block }), last)
            },
            _ => (None, 0),
          }
        },
        None => (None, 0),
      }
    },
    &Token::ID(ref id) => {
      let mut index = start + 1;
      match get_token(tokens, index) {
        &Token::Colon => {
          index += 1;
          let (block, change) = parse_block(tokens, index);
          (Some(Definition { id: id.clone(), params: Vec::new(),
                             block: block }), change)
        },
        &Token::OpenParen => {
          let (opt, change) = parse_params(tokens, index + 1);
          match opt {
            Some(params) => {
              index = change;
              match get_token(tokens, index) {
                &Token::Colon => {
                  index += 1;
                  let (block, last) = parse_block(tokens, index);
                  (Some(Definition { id: id.clone(), params: params,
                                     block: block }), last)
                },
                _ => (None, 0),
              }
            },
            None => (None, 0),
          }
        },
        _ => (None, 0),
      }
    },
    _ => (None, 0),
  }
}

fn parse_call(tokens: &Vec<Token>, start: usize) -> (Call, usize) {
  let id = match get_token(tokens, start) {
    &Token::ID(ref s) => s.clone(),
    _ => panic!("this should never happen"),
  };
  let mut rc = Call { id: id, params: Vec::new() };
  let mut index = start + 1;
  match get_token(tokens, index) {
    &Token::OpenParen => {
      index += 1;
      loop {
        match get_token(tokens, index) {
          &Token::CloseParen => {
            index += 1;
            break;
          },
          _ => {
            let (param, change) = parse_next_expression(tokens, index);
            match param {
              Some(exp) => rc.params.push(exp),
              // TODO: better
              None => panic!("error parsing call")
            }
            index = change;
            match get_token(tokens, index) {
              &Token::Comma => {
                index += 1;
              },
              &Token::CloseParen => {
                // do nothing, will be caught at beginning of next loop
              },
              _ => panic!("comma or close paren expected")
            }
          }
        }
      }
    },
    _ => {
      // Do nothing, bare function call
    },
  }
  (rc, index)
}

fn parse_list(tokens: &Vec<Token>, start: usize) -> (List, usize) {
  let mut rc = List { items: Vec::new() };
  let mut index = start + 1;
  loop {
    match get_token(tokens, index) {
      &Token::CloseBracket => {
        break;
      },
      _ => {
        let (item, change) = parse_next_expression(tokens, index);
        match item {
          Some(exp) => rc.items.push(exp),
          None => panic!("error parsing list"),
        }
        index = change;
        match get_token(tokens, index) {
          &Token::Comma => {
            index += 1;
          },
          &Token::CloseBracket => {
            // do nothing, will be caught at beginning of next loop
          },
          _ => panic!("comma or close bracket expected")
        }
      },
    }
  }
  (rc, index + 1)
}

fn parse_next_expression(tokens: &Vec<Token>, start: usize) ->
  (Option<Expression>, usize) {
  match get_token(tokens, start) {
    &Token::Nil => (Some(Expression::Nil), start + 1),
    &Token::True => (Some(Expression::True), start + 1),
    &Token::False => (Some(Expression::False), start + 1),
    &Token::Integer(x) => (Some(Expression::Integer(x)), start + 1),
    &Token::Float(x) => (Some(Expression::Float(x)), start + 1),
    &Token::String(ref s) => (Some(Expression::String(s.clone())), start + 1),
    &Token::OpenBracket => {
      let (list, index) = parse_list(tokens, start);
      (Some(Expression::List(list)), index)
    },
    &Token::ID(_) => {
      let (opt, index) = parse_definition(tokens, start);
      match opt {
        Some(def) => {
          (Some(Expression::Definition(def)), index - 1)
        },
        None => {
          let (call, index) = parse_call(tokens, start);
          (Some(Expression::Call(call)), index)
        },
      }
    },
    &Token::Colon | &Token::OpenParen => {
      let (opt, index) = parse_definition(tokens, start);
      match opt {
        Some(def) => {
          (Some(Expression::Definition(def)), index - 1)
        },
        None => {
          // TODO: better
          panic!("expected function definition, didn't get one");
        },
      }
    },
    _ => (None, 0),
  }
}

fn parse_block(tokens: &Vec<Token>, start: usize) -> (Block, usize) {
  let mut rc = Block { expressions: Vec::new() };
  let mut index = start;
  loop {
    let (next, change) = parse_next_expression(tokens, index);
    match next {
      Some(value) => {
        index = change;
        // Debug output:
        //println!("{:?}", value);
        rc.expressions.push(value);
      },
      None => {
        index += 1;
        break;
      },
    }
    let check = get_token(tokens, index);
    match check {
      &Token::Semicolon => {
        // do nothing
      },
      _ => {
        panic!("semicolon expected after expression");
      },
    }
    index += 1;
  }
  (rc, index)
}

pub fn parse(tokens: &Vec<Token>) -> Block {
  let (block, index) = parse_block(&tokens, 0);
  if index < tokens.len() {
    // TODO: better error handling here
    panic!("syntax error, unexpected token")
  }
  block
}

// Super simple tokenizer/scanner:

use encoding::Token;

fn next_token(chars: &Vec<char>, start: usize) -> (Token, usize) {
  let reserved = [':', ';', ',', '(', ')', '[', ']', '{', '}', '"', '#'];

  let mut index = start;
  let mut c = chars[index];
  while c.is_whitespace() {
    if index == chars.len() - 1 {
      // EOF is only returned with trailing whitespace (or closing comment), but
      // we need to return something when there's no "real" token left to return
      return (Token::EOF, index + 1);
    }
    index += 1;
    c = chars[index];
  }
  let from = index;
  match c {
    ':' => return (Token::Colon, index + 1),
    ';' => return (Token::Semicolon, index + 1),
    ',' => return (Token::Comma, index + 1),
    '(' => return (Token::OpenParen, index + 1),
    ')' => return (Token::CloseParen, index + 1),
    '[' => return (Token::OpenBracket, index + 1),
    ']' => return (Token::CloseBracket, index + 1),
    '{' => return (Token::OpenBrace, index + 1),
    '}' => return (Token::CloseBrace, index + 1),
    '#' => {
      index += 1;
      c = chars[index];
      while index < chars.len() - 1 && c != '\n' && c != '\r' {
        index += 1;
        c = chars[index];
      }
      // This is a comment, so we return the next token after it
      next_token(&chars, index)
    }
    '"' => {
      index += 1;
      c = chars[index];
      while index < chars.len() - 1 && c != '"' {
        index += 1;
        c = chars[index];
      }
      let s = chars[from + 1..index].iter().cloned().collect();
      if c != '"' {
        // TODO: Do this in a more controlled way
        panic!("Unterminated string in source: {}", s);
      }
      (Token::String(s), index + 1)
    }
    _ => {
      while index < chars.len() - 1 && !c.is_whitespace() &&
        !reserved.contains(&c) {
        index += 1;
        c = chars[index];
      }
      let s:String = chars[from..index].iter().cloned().collect();
      if s == "true" {
        return (Token::True, index);
      } else if s == "false" {
        return (Token::False, index);
      } else if s == "nil" {
        return (Token::Nil, index);
      }
      match s.parse::<i64>() {
        Ok(n) => (Token::Integer(n), index),
        _ => {
          match s.parse::<f64>() {
            Ok(n) => (Token::Float(n), index),
            _ => (Token::ID(s), index),
          }
        },
      }
    },
  }
}

pub fn tokenize(s: &str) -> Vec<Token> {
  let chars:Vec<char> = s.chars().collect();

  let mut tokens = Vec::new();

  let mut index = 0;
  while index < chars.len() {
    let (token, change) = next_token(&chars, index);
    index = change;
    // Debug output:
    //println!("{}:{:?}", index, token);
    tokens.push(token);
  }
  tokens
}

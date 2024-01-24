use std::ops::Range;

use crate::error::{Error, Result};
use crate::eval::{Op, Roll};

impl Roll {
  fn builder() -> RollBuilder {
    RollBuilder {
      ops: Vec::with_capacity(64),
      stack: Stack::default(),
    }
  }
}

struct RollBuilder {
  ops: Vec<Op>,
  stack: Stack,
}

impl RollBuilder {
  fn finish(self) -> Roll {
    Roll {
      ops: self.ops,
      stack_size: self.stack.max,
    }
  }

  fn emit(&mut self, op: Op) {
    op.stack_effects(&mut self.stack);
    self.ops.push(op);
  }
}

impl Op {
  fn stack_effects(&self, stack: &mut Stack) {
    match self {
      Op::Num(_) => {
        stack.push(1);
      }
      Op::Add => {
        stack.pop(2);
        stack.push(1);
      }
      Op::Sub => {
        stack.pop(2);
        stack.push(1);
      }
      Op::Mul => {
        stack.pop(2);
        stack.push(1);
      }
      Op::Div => {
        stack.pop(2);
        stack.push(1);
      }
      Op::Neg => {
        stack.pop(1);
        stack.push(1);
      }
      Op::Dice => {
        stack.pop(2);
        stack.push(1);
      }
    }
  }
}

#[derive(Default)]
struct Stack {
  current: usize,
  max: usize,
}

impl Stack {
  fn push(&mut self, n: usize) {
    self.current += n;
    self.max = std::cmp::max(self.max, self.current);
  }

  fn pop(&mut self, n: usize) {
    self.current -= n;
  }
}

pub fn parse(input: &str) -> Result<Roll> {
  let mut lex = Lexer::new(input);
  let mut roll = Roll::builder();
  parse_expr(&mut lex, &mut roll)?;
  Ok(roll.finish())
}

fn parse_expr(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  parse_term(lex, roll)
}

fn parse_term(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  parse_factor(lex, roll)?;
  loop {
    let op = match lex.current() {
      Token::Plus => Op::Add,
      Token::Minus => Op::Sub,
      _ => break,
    };
    lex.bump()?;
    parse_factor(lex, roll)?;
    roll.ops.push(op);
  }
  Ok(())
}

fn parse_factor(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  parse_prefix(lex, roll)?;
  loop {
    let op = match lex.current() {
      Token::Star => Op::Mul,
      Token::Slash => Op::Div,
      _ => break,
    };
    lex.bump()?;
    parse_prefix(lex, roll)?;
    roll.emit(op);
  }
  Ok(())
}

fn parse_prefix(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  if lex.bump_if(Token::Minus)? {
    parse_dice_unary(lex, roll)?;
    roll.emit(Op::Neg);
    return Ok(());
  }
  parse_dice_unary(lex, roll)
}

fn parse_dice_unary(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  if lex.bump_if(Token::Dice)? {
    roll.emit(Op::Num(1));
    parse_primary(lex, roll)?;
    roll.emit(Op::Dice);
    return Ok(());
  }
  parse_dice_binary(lex, roll)
}

fn parse_dice_binary(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  parse_primary(lex, roll)?;
  if lex.bump_if(Token::Dice)? {
    parse_primary(lex, roll)?;
    roll.emit(Op::Dice)
  }
  Ok(())
}

fn parse_primary(lex: &mut Lexer, roll: &mut RollBuilder) -> Result<()> {
  if lex.check(Token::Number) {
    lex.bump()?;
    roll.emit(Op::Num(parse_int(lex.prev_slice())?));
    return Ok(());
  }

  if lex.bump_if(Token::ParenL)? {
    let mut n = 1;
    while lex.bump_if(Token::ParenL)? {
      n += 1;
    }

    parse_expr(lex, roll)?;

    for _ in 0..n {
      lex.expect(Token::ParenR)?;
    }
    return Ok(());
  }

  if lex.check(Token::Eof) {
    Err(Error::new("unexpected end of input"))
  } else {
    Err(Error::new(format!("unexpected input: {:?}", lex.slice())))
  }
}

struct Lexer<'a> {
  input: &'a str,
  inner: logos::Lexer<'a, Token>,
  current: Token,
  previous: Token,
  previous_span: Range<usize>,
  eof: Token,
}

impl<'a> Lexer<'a> {
  fn new(input: &'a str) -> Self {
    let eof = Token::Eof;
    let mut inner = logos::Lexer::new(input);
    Self {
      input,
      current: inner.next().unwrap_or(eof),
      previous: eof,
      previous_span: input.len()..input.len(),
      eof,
      inner,
    }
  }

  fn slice(&self) -> &str {
    self.inner.slice()
  }

  fn prev_slice(&self) -> &str {
    &self.input[self.previous_span.clone()]
  }

  fn current(&self) -> &Token {
    &self.current
  }

  fn expect(&mut self, t: Token) -> Result<()> {
    if self.bump_if(t)? {
      Ok(())
    } else {
      let s = self.slice();
      let e = if s.trim().is_empty() {
        Error::new(format!("expected {:?}, got end of input", t.as_str()))
      } else {
        Error::new(format!("expected {:?}, got {s:?}", t.as_str()))
      };
      Err(e)
    }
  }

  fn check(&self, t: Token) -> bool {
    use std::mem::discriminant;
    discriminant(&self.current) == discriminant(&t)
  }

  fn bump(&mut self) -> Result<&Token> {
    std::mem::swap(&mut self.previous, &mut self.current);
    self.previous_span = self.inner.span();
    self.current = self.inner.next().unwrap_or(self.eof);

    if self.current == Token::Error {
      let mut error_span = self.inner.span();
      while matches!(self.inner.next().unwrap_or(self.eof), Token::Error) {
        error_span = join_spans(error_span, self.inner.span());
      }
      return Err(Error::new(format!(
        "unexpected input: {:?}",
        &self.input[error_span]
      )));
    }
    Ok(&self.previous)
  }

  fn bump_if(&mut self, t: Token) -> Result<bool> {
    if self.check(t) {
      self.bump()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }
}

#[derive(PartialEq, Clone, Copy, logos::Logos)]
enum Token {
  #[token("+")]
  Plus,
  #[token("-")]
  Minus,
  #[token("*")]
  Star,
  #[token("/")]
  Slash,
  #[token("d")]
  Dice,

  #[token("~")]
  RangeTilde,
  #[token("..")]
  RangeDot2,
  #[token("...")]
  RangeDot3,

  #[token("(")]
  ParenL,
  #[token(")")]
  ParenR,

  #[regex(r"[0-9]+")]
  Number,

  #[regex(r"\s+", logos::skip)]
  #[error]
  Error,
  Eof,
}

impl Token {
  fn as_str(&self) -> &'static str {
    match self {
      Token::Plus => "+",
      Token::Minus => "-",
      Token::Star => "*",
      Token::Slash => "/",
      Token::Dice => "d",
      Token::RangeTilde => "~",
      Token::RangeDot2 => "..",
      Token::RangeDot3 => "...",
      Token::ParenL => "(",
      Token::ParenR => ")",
      Token::Number => "number",
      Token::Error => "<error>",
      Token::Eof => "<eof>",
    }
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

fn join_spans(a: Range<usize>, b: Range<usize>) -> Range<usize> {
  a.start..b.end
}

fn parse_int(v: &str) -> Result<i64> {
  match v.parse() {
    Ok(v) => Ok(v),
    Err(e) => Err(Error::new(format!("invalid integer {v:?}: {e}"))),
  }
}

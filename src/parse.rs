use std::ops::Range;

use crate::eval::Op;
use crate::Result;

pub fn parse(input: &str) -> Result<Vec<Op>> {
  let mut lex = Lexer::new(input);
  let mut out = Vec::new();
  parse_expr(&mut lex, &mut out)?;
  Ok(out)
}

fn parse_expr(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  parse_term(lex, out)
}

fn parse_term(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  parse_factor(lex, out)?;
  loop {
    let op = match lex.current() {
      Token::Plus => Op::Add,
      Token::Minus => Op::Sub,
      _ => break,
    };
    lex.bump()?;
    parse_factor(lex, out)?;
    out.push(op);
  }
  Ok(())
}

fn parse_factor(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  parse_prefix(lex, out)?;
  loop {
    let op = match lex.current() {
      Token::Star => Op::Mul,
      Token::Slash => Op::Div,
      _ => break,
    };
    lex.bump()?;
    parse_prefix(lex, out)?;
    out.push(op);
  }
  Ok(())
}

fn parse_prefix(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  if lex.bump_if(Token::Minus)? {
    parse_dice_unary(lex, out)?;
    out.push(Op::Neg);
    return Ok(());
  }
  parse_dice_unary(lex, out)
}

fn parse_dice_unary(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  if lex.bump_if(Token::Dice)? {
    out.push(Op::Num(1));
    parse_primary(lex, out)?;
    out.push(Op::Dice);
    return Ok(());
  }
  parse_dice_binary(lex, out)
}

fn parse_dice_binary(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  parse_primary(lex, out)?;
  if lex.bump_if(Token::Dice)? {
    parse_primary(lex, out)?;
    out.push(Op::Dice)
  }
  Ok(())
}

fn parse_primary(lex: &mut Lexer, out: &mut Vec<Op>) -> Result<()> {
  if lex.check(Token::Number) {
    lex.bump()?;
    out.push(Op::Num(parse_int(lex.prev_slice())?));
    return Ok(());
  }

  if lex.bump_if(Token::ParenL)? {
    let mut n = 1;
    while lex.bump_if(Token::ParenL)? {
      n += 1;
    }

    parse_expr(lex, out)?;

    for _ in 0..n {
      lex.expect(Token::ParenR)?;
    }
    return Ok(());
  }

  if lex.check(Token::Eof) {
    Err(format!("Weirdga missing input"))
  } else {
    Err(format!("Weirdga 👉 `{}` ❓", lex.slice()))
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
    &self.input[self.inner.span()]
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
      Err(format!("Weirdga missing `{}`", t))
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
      return Err(format!("Weirdga 👉 `{}` ❓", &self.input[error_span]));
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

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Token::Plus => "+",
      Token::Minus => "-",
      Token::Star => "*",
      Token::Slash => "/",
      Token::Dice => "d",
      Token::ParenL => "(",
      Token::ParenR => ")",
      Token::Number => "number",
      Token::Error => "<error>",
      Token::Eof => "<eof>",
    };
    write!(f, "{str}")
  }
}

fn join_spans(a: Range<usize>, b: Range<usize>) -> Range<usize> {
  a.start..b.end
}

fn parse_int(slice: &str) -> Result<i64, String> {
  slice
    .parse()
    .map_err(|_| format!("Weirdga 👉 `{}` ❓", slice))
}

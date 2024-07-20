use anyhow::Context;
use roll_dice::eval::Roll;
use roll_dice::parse::parse;
use roll_dice::rng::Prng;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn random() -> u64 {
  let mut bytes = [0u8; 8];
  getrandom::getrandom(&mut bytes).unwrap();
  u64::from_le_bytes(bytes)
}

fn main() -> Result<()> {
  let mut seed = random();
  let mut rng = Prng::new(seed);

  // `()` can be used when no completer is required
  let mut rl = DefaultEditor::new()?;

  loop {
    match rl.readline("> ") {
      Ok(line) => {
        let cmd = match parse_cmd(&line) {
          Ok(cmd) => cmd,
          Err(e) => {
            eprintln!("{e}");
            continue;
          }
        };

        match cmd {
          Command::PrintSeed => {
            println!("{seed}");
          }
          Command::SetSeed(Some(v)) => {
            seed = v;
            rng = Prng::new(seed);
          }
          Command::SetSeed(None) => {
            rng = Prng::new(random());
          }
          Command::Eval(roll) => match roll.eval_with_rng(u64::MAX, &rng) {
            Ok(value) => println!("{value}"),
            Err(e) => eprintln!("{e}"),
          },
        }

        rl.add_history_entry(line)?;
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        println!("Error: {:?}", err);
        break;
      }
    }
  }

  Ok(())
}

fn parse_cmd(line: &str) -> anyhow::Result<Command> {
  if let Some(line) = line.strip_prefix("!seed") {
    let line = line.trim();
    if line.is_empty() {
      return Ok(Command::PrintSeed);
    }

    if line == "none" {
      return Ok(Command::SetSeed(None));
    }

    let seed = line.trim().parse().context("failed to parse seed")?;
    return Ok(Command::SetSeed(Some(seed)));
  }

  let roll = parse(line).context("failed to parse roll")?;
  Ok(Command::Eval(roll))
}

enum Command {
  PrintSeed,
  SetSeed(Option<u64>),
  Eval(Roll),
}

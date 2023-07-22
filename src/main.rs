mod meeting;

use clap::Parser;
use dialoguer::Select;

use crate::meeting::Meeting;

#[derive(Debug, Parser)]
struct Cli {
  #[arg(short, long, env = "ZOOOM_SOURCE")]
  source: Vec<String>,

  #[arg(long, default_value_t = 0, env = "ZOOOM_BUFFER_START")]
  buffer_start: i64,

  #[arg(long, default_value_t = 0, env = "ZOOOM_BUFFER_END")]
  buffer_end: i64,
}

fn main() {
  let cli = Cli::parse();

  let meets = cli
    .source
    .iter()
    .flat_map(Meeting::read_meetings)
    .flatten()
    .filter(|m| m.is_now(cli.buffer_start, cli.buffer_end))
    .collect::<Vec<_>>();

  let meet = &meets[match meets.len() {
    0 => {
      eprintln!("No Meeting Found!");
      return;
    }
    1 => 0,
    _ => match Select::new()
      .with_prompt("Choose Meeting")
      .items(&meets)
      .interact_opt()
    {
      Ok(i) => {
        if let Some(i) = i {
          i
        } else {
          println!("No Meeting Choosen!");
          return;
        }
      }
      Err(err) => {
        eprintln!("Error Choosing Meeting: {err}");
        return;
      }
    },
  }];

  if let Err(err) = meet.join() {
    println!("Error Joining Meeting: {meet} -> {err}");
  } else {
    println!("Joining Meeting: {meet}");
  }
}

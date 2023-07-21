mod meeting;

use clap::Parser;

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

fn choose(meets: &[Meeting]) -> &Meeting {
  &meets[0]
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

  if meets.is_empty() {
    eprintln!("No Meeting Found!");
  } else if let Err(err) = choose(&meets).join() {
    eprintln!("Error Joining Meeting: {err}");
  } else {
    println!("Joining Meeting!");
  }
}

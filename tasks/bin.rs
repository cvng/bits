use anyhow::Result;
use std::env;
use xshell::cmd;
use xshell::Shell;

fn main() -> Result<()> {
  let sh = Shell::new()?;

  let args = env::args().collect::<Vec<_>>();
  let cmd = &args[1];

  cmd!(sh, "tasks/{cmd}.sh").run()?;

  Ok(())
}

use std::env;
use xshell::cmd;
use xshell::Shell;

fn main() -> anyhow::Result<()> {
  let args = env::args().collect::<Vec<_>>();

  let sh = Shell::new()?;
  let cmd = &args[1];

  cmd!(sh, "tasks/{cmd}.sh").run()?;

  Ok(())
}

use std::env;
use xshell::cmd;
use xshell::Shell;

fn main() -> anyhow::Result<()> {
  let args = env::args().collect::<Vec<_>>();

  let sh = Shell::new()?;
  let cmd = format!("tasks/{}.sh", args[1]);

  cmd!(sh, "{cmd}").run()?;

  Ok(())
}

use std::env;
use std::process;

fn main() {
  println!(
    "{}",
    String::from_utf8_lossy(
      &process::Command::new("sh")
        .current_dir("tasks")
        .arg(env::args().collect::<Vec<_>>().last().unwrap().to_owned() + ".sh")
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
        .unwrap()
        .stdout
    )
  );
}

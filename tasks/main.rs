fn main() {
  println!(
    "{}",
    String::from_utf8_lossy(
      &std::process::Command::new("sh")
        .arg(format!(
          "./tasks/{}.sh",
          std::env::args().collect::<Vec<_>>().last().unwrap()
        ))
        .output()
        .unwrap()
        .stdout
    )
  );
}

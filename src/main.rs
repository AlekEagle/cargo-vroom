// cargo-vroom: Car no go run, car go vroom!
use std::process::Command;

fn main() {
  let args = std::env::args().skip(1).collect::<Vec<_>>();
  let mut cmd = Command::new("cargo");
  cmd.arg("run");
  cmd.args(&args);
  // Pipe the output of the child process to the parent process.
  cmd.stdout(std::process::Stdio::inherit());
  cmd.stderr(std::process::Stdio::inherit());
  // Pipe the input of the parent process to the child process.
  cmd.stdin(std::process::Stdio::inherit());
  let status = cmd.status().unwrap();
  std::process::exit(status.code().unwrap());
}

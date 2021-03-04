use std::env::var;
use std::io::Write;
use std::io::stderr;
use std::io::stdout;
use std::process::exit;
use std::process::Command;
use std::process::Output;

const DEFAULT_REMOTE: &str = "origin";
const DEFAULT_BRANCH: &str = "default";

fn main() {
    // Git version
    let command = "git --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap();
    // Initialize repository
    let command = format!("git init --initial-branch={}", DEFAULT_BRANCH);
    exec(command.as_str());
    // Add remote
    let server = var("GITHUB_SERVER_URL").unwrap();
    let repository = var("GITHUB_REPOSITORY").unwrap();
    let command = format!(
        "git remote add {} {}/{}",
        DEFAULT_REMOTE, server, repository
    );
    exec(command.as_str());
    // Fetch origin
    let refspec = var("GITHUB_REF").unwrap();
    let command = format!(
        "git fetch \
        --depth=1 \
        --no-tags \
        --update-head-ok \
        {} +{}:{}",
        DEFAULT_REMOTE, refspec, DEFAULT_BRANCH
    );
    exec(command.as_str());
    // Checkout branch
    let command = "git checkout";
    exec(command);
    // Show
    let command = "git --no-pager log --date=iso --no-decorate";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap();
}

fn exec(command: &str) -> Output {
    println!("❯ {}", command);
    let split: Vec<_> = command.trim().split_whitespace().collect();
    let (executable, arguments) = split.split_first().unwrap();
    let output = Command::new(executable).args(arguments).output().unwrap();
    if output.status.success() {
        output
    } else {
        eprintln!("fail: {}", command);
        eprintln!("--- status ---");
        eprintln!("{}", output.status);
        eprintln!();
        eprintln!("--- stdout ---");
        stderr().write_all(&output.stdout).unwrap();
        eprintln!();
        eprintln!("--- stderr ---");
        stderr().write_all(&output.stderr).unwrap();
        eprintln!();
        exit(1);
    }
}

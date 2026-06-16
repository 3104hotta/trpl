// 使い方:
// cd ch04-understanding-ownership
// cargo run                       # → 全bin（01_ownership → 02 → 03）を順に実行
// cargo run -- 01_ownership       # → 個別実行
// cargo run -- 01_ownership 03_slices  # → 複数指定可
use std::process::{Command, exit};

const BINS: &[&str] = &[
    "01_ownership",
    "02_references_borrowing",
    "03_slices",
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let targets: Vec<&str> = if args.is_empty() {
        BINS.to_vec()
    } else {
        for name in &args {
            if !BINS.contains(&name.as_str()) {
                eprintln!("Unknown bin: {name}");
                eprintln!("Available: {}", BINS.join(", "));
                exit(1);
            }
        }
        args.iter().map(|s| s.as_str()).collect()
    };

    for bin in targets {
        println!("=== {bin} ===");
        let status = Command::new("cargo")
            .args(["run", "--quiet", "--bin", bin])
            .status()
            .expect("failed to spawn cargo");
        if !status.success() {
            exit(status.code().unwrap_or(1));
        }
        println!();
    }
}

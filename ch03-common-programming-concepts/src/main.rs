// 使い方:
// cd ch03-common-programming-concepts
// cargo run                       # → 全bin（01_variables → 02_data_types）を順に実行
// cargo run -- 01_variables       # → 個別実行
// cargo run -- 01_variables 02_data_types  # → 複数指定可
use std::process::{Command, exit};

const BINS: &[&str] = &[
    "01_variables",
    "02_data_types",
    "03_functions",
    "04_1_branches",
    "04_2_loops",
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

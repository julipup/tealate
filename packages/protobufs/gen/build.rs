use std::process::{Command, exit};

fn main() {
    println!("cargo:rerun-if-changed=../tealate");

    // Running npx nx run protobufs:generate command
    let status = Command::new("npx")
        .args(["nx", "run", "protobufs:generate"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }
}
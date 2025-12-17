// ビルドスクリプト
// タスク:
// - boot.s をnasmでコンパイルし, cargoがリンクできるようにする
// - boot.s を監視するようcargoに伝える
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // - boot.s をnasmでコンパイルし, cargoがリンクできるようにする
    let status = Command::new("nasm")
        .args(&["-f", "elf32", "src/boot/boot.s", "-o"])
        .arg(out_dir.join("boot.o"))
        .status()
        .expect("failed to run nasm");

    if !status.success() {
        panic!("nasm failed");
    }

    let status = Command::new("ar")
        .args(&["crus", out_dir.join("libboot.a").to_str().unwrap()])
        .arg(out_dir.join("boot.o"))
        .status()
        .expect("failed to run ar");

    if !status.success() {
        panic!("ar failed");
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=boot");

    // - boot.s を監視するようcargoに伝える
    println!("cargo:rerun-if-changed=src/boot/boot.s");
    println!("cargo:rerun-if-changed=linker.ld");
}

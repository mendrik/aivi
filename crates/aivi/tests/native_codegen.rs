use std::process::Command;

use aivi::{compile_rust_native, desugar_target};
use tempfile::tempdir;

#[test]
fn native_codegen_smoke_compiles_and_runs() {
    let dir = tempdir().expect("tempdir");
    let source_path = dir.path().join("main.aivi");
    std::fs::write(
        &source_path,
        r#"module app.main
main : Effect Text Unit
main = effect {
  _ <- print "Hello from AIVI!"
  pure Unit
}
"#,
    )
    .expect("write aivi source");

    let source_path_str = source_path.to_string_lossy().to_string();
    let program = desugar_target(&source_path_str).expect("desugar");
    let rust = compile_rust_native(program).expect("compile_rust_native");
    assert!(rust.contains("fn main()"));
    assert!(!rust.contains("PROGRAM_JSON"));

    let rust_path = dir.path().join("main.rs");
    std::fs::write(&rust_path, rust).expect("write rust source");

    let out_path = dir.path().join("aivi_native_out");
    let output = Command::new("rustc")
        .arg(&rust_path)
        .arg("--edition=2021")
        .arg("-o")
        .arg(&out_path)
        .output()
        .expect("spawn rustc");
    assert!(
        output.status.success(),
        "rustc failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let output = Command::new(&out_path)
        .output()
        .expect("run native output");
    assert!(
        output.status.success(),
        "native output failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Hello from AIVI!"),
        "unexpected stdout: {stdout}"
    );
}


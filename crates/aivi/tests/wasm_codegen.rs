use std::path::PathBuf;

use aivi::{compile_wasm, desugar_target};

fn set_workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");
    std::env::set_current_dir(&workspace_root).expect("set cwd");
    workspace_root.to_path_buf()
}

#[test]
fn compile_wasm_example_has_header() {
    let _root = set_workspace_root();
    let program = desugar_target("examples/10_wasm.aivi").expect("desugar");
    let wasm = compile_wasm(program).expect("compile wasm");
    assert!(wasm.starts_with(b"\0asm"));
    assert!(wasm.len() > 8);
}

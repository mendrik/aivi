use crate::hir::HirProgram;
use crate::{kernel, rust_ir, rustc_backend};
use crate::AiviError;

/// Legacy backend: embed HIR as JSON and run via `aivi::run_native`.
pub fn compile_rust(program: HirProgram) -> Result<String, AiviError> {
    let json = serde_json::to_vec(&program)
        .map_err(|err| AiviError::Codegen(format!("failed to serialize program: {err}")))?;
    let escaped = escape_bytes(&json);

    let mut output = String::new();
    output.push_str("use aivi::HirProgram;\n\n");
    output.push_str("const PROGRAM_JSON: &[u8] = b\"");
    output.push_str(&escaped);
    output.push_str("\";\n\n");
    output.push_str("fn build_program() -> HirProgram {\n");
    output.push_str("    serde_json::from_slice(PROGRAM_JSON)\n");
    output.push_str("        .expect(\"deserialize embedded AIVI program\")\n");
    output.push_str("}\n\n");
    output.push_str("fn main() {\n");
    output.push_str("    let program = build_program();\n");
    output.push_str("    if let Err(err) = aivi::run_native(program) {\n");
    output.push_str("        eprintln!(\"{err}\");\n");
    output.push_str("        std::process::exit(1);\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    Ok(output)
}

/// Legacy backend: embed HIR as JSON and expose `run()` via `aivi::run_native`.
pub fn compile_rust_lib(program: HirProgram) -> Result<String, AiviError> {
    let json = serde_json::to_vec(&program)
        .map_err(|err| AiviError::Codegen(format!("failed to serialize program: {err}")))?;
    let escaped = escape_bytes(&json);

    let mut output = String::new();
    output.push_str("use aivi::HirProgram;\n\n");
    output.push_str("const PROGRAM_JSON: &[u8] = b\"");
    output.push_str(&escaped);
    output.push_str("\";\n\n");
    output.push_str("pub fn build_program() -> HirProgram {\n");
    output.push_str("    serde_json::from_slice(PROGRAM_JSON)\n");
    output.push_str("        .expect(\"deserialize embedded AIVI program\")\n");
    output.push_str("}\n\n");
    output.push_str("pub fn run() -> Result<(), aivi::AiviError> {\n");
    output.push_str("    aivi::run_native(build_program())\n");
    output.push_str("}\n");

    Ok(output)
}

/// Experimental backend: lower to Kernel -> Rust IR and emit standalone Rust.
///
/// Limitations are those of `rust_ir` + `rustc_backend` (e.g. `match` not supported yet).
pub fn compile_rust_native(program: HirProgram) -> Result<String, AiviError> {
    let kernel = kernel::lower_hir(strip_stdlib_modules(program));
    let rust_ir = rust_ir::lower_kernel(kernel)?;
    rustc_backend::emit_rustc_source(rust_ir)
}

/// Experimental backend: emit a Rust library with exported definitions.
pub fn compile_rust_native_lib(program: HirProgram) -> Result<String, AiviError> {
    let kernel = kernel::lower_hir(strip_stdlib_modules(program));
    let rust_ir = rust_ir::lower_kernel(kernel)?;
    rustc_backend::emit_rustc_source_lib(rust_ir)
}

fn strip_stdlib_modules(mut program: HirProgram) -> HirProgram {
    // The embedded stdlib is used for parsing/typechecking today, but in the interpreter
    // runtime those values are provided via Rust builtins. The native codegen backend
    // currently only supports a small builtin subset, so we avoid emitting stdlib modules.
    program.modules.retain(|m| !m.name.starts_with("aivi"));
    program
}

fn escape_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    for byte in bytes {
        for escaped in std::ascii::escape_default(*byte) {
            out.push(escaped as char);
        }
    }
    out
}
